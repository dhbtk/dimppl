use crate::database::Pool;
use crate::error_handling::AppResult;
use crate::models::podcast::{SyncStateRequest, SyncStateResponse};
use crate::models::{podcast, user_device};
use axum::extract::State;
use axum::headers::HeaderMap;
use axum::Json;

pub async fn sync_state(
    State(pool): State<Pool>,
    headers: HeaderMap,
    Json(sync_state_request): Json<SyncStateRequest>,
) -> AppResult<Json<SyncStateResponse>> {
    let mut conn = pool.get().await?;
    let (user, device) =
        user_device::user_and_device_from_http_request(&headers, &mut conn).await?;
    tracing::debug!(
        "Starting sync for user id={} device name={}",
        user.id,
        device.name
    );
    // TODO: maybe lock by user so this can't run in parallel with another sync operation
    for podcast in &sync_state_request.podcasts {
        tracing::debug!("Syncing podcast guid={} url={}", podcast.guid, podcast.url);
        let result = podcast::sync_upsert_podcast(&user, podcast, &mut conn).await?;
        tracing::debug!("Sync result: {:#?}", result);
    }
    for (guid, episodes) in &sync_state_request.episodes {
        tracing::debug!(
            "Syncing {} episodes for podcast guid {}",
            episodes.len(),
            guid
        );
        podcast::sync_upsert_episodes(&user, guid, episodes, &mut conn).await?;
    }
    Ok(Json(podcast::get_sync_response(&user, &mut conn).await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::create_test_app;
    use crate::models::podcast::{SyncPodcast, SyncPodcastEpisode};
    use crate::models::user_device::test_user_and_device;
    use axum::http;
    use axum::http::{Request, StatusCode};
    use chrono::{Local, NaiveDateTime};
    use hyper::Body;
    use serial_test::serial;
    use std::collections::HashMap;
    use tower::ServiceExt;

    #[tokio::test]
    #[serial]
    pub async fn test_sync_state() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, device) = test_user_and_device(&mut conn).await.unwrap();
        let new_podcast = SyncPodcast {
            url: "https://google.com".into(),
            guid: "guid".into(),
            deleted_at: None,
            updated_at: Local::now().naive_utc(),
        };
        let episodes = vec![
            SyncPodcastEpisode {
                guid: "ep1".into(),
                url: "https://ep1.changed".into(),
                listened_seconds: 0,
                completed: false,
                updated_at: Local::now().naive_utc(),
            },
            SyncPodcastEpisode {
                guid: "ep2".into(),
                url: "https://ep2.changed".into(),
                listened_seconds: 500,
                completed: true,
                updated_at: Local::now().naive_utc(),
            },
            SyncPodcastEpisode {
                guid: "ep3".into(),
                url: "https://ep3".into(),
                listened_seconds: 350,
                completed: false,
                updated_at: Local::now().naive_utc(),
            },
        ];
        let mut episode_map = HashMap::new();
        episode_map.insert(new_podcast.guid.clone(), episodes);
        let payload = SyncStateRequest {
            podcasts: vec![new_podcast.clone()],
            episodes: episode_map,
        };

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/sync")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", device.access_token))
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body: SyncStateResponse = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!("guid", response_body.podcasts[0].guid);
        assert_eq!("ep1", response_body.episodes["guid"][0].guid);
        assert_eq!("ep2", response_body.episodes["guid"][1].guid);
        assert_eq!("ep3", response_body.episodes["guid"][2].guid);
    }
}
