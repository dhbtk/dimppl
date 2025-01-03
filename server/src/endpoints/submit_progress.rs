use crate::database::Pool;
use crate::error_handling::AppResult;
use crate::models::{episode, user_device};
use axum::extract::State;
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use dimppl_shared::progress::ProgressUpdateRequest;

#[debug_handler]
pub async fn submit_progress(
    State(pool): State<Pool>,
    headers: HeaderMap,
    Json(request): Json<ProgressUpdateRequest>,
) -> AppResult<(StatusCode, ())> {
    let mut conn = pool.get().await?;
    let user = user_device::user_from_http_request(&headers, &mut conn).await?;
    episode::update_progress(user.id, request, &mut conn).await?;
    Ok((StatusCode::OK, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::create_test_app;
    use crate::models::podcast::test_podcast_with_episodes;
    use crate::models::user_device::test_user_and_device;
    use crate::models::PodcastEpisode;
    use axum::http::Request;
    use chrono::Local;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use hyper::{http, Body};
    use serial_test::serial;
    use tower::ServiceExt;

    #[serial]
    #[tokio::test]
    async fn test_update_progress_successful_update() {
        let (state, app) = create_test_app();
        let mut conn = state.pool.get().await.unwrap();
        let (user, device) = test_user_and_device(&mut conn).await.unwrap();
        let (existing_podcast, episodes) = test_podcast_with_episodes(&user, &mut conn).await.unwrap();

        let request = ProgressUpdateRequest {
            podcast_guid: existing_podcast.guid.clone(),
            episode_guid: episodes[1].guid.clone(),
            listened_seconds: 250,
            completed: true,
            updated_at: Local::now().naive_utc(),
        };
        
        let web_request = Request::builder()
            .method(http::Method::POST)
            .uri("/submit_progress")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", device.access_token))
            .body(Body::from(serde_json::to_string(&request).unwrap()))
            .unwrap();

        let response = app.oneshot(web_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let episode = {
            use crate::schema::podcast_episodes::dsl::*;
            podcast_episodes
                .filter(guid.eq(&episodes[1].guid))
                .select(PodcastEpisode::as_select())
                .load(&mut conn)
                .await.unwrap()
                .remove(0)
        };
        assert_eq!(250, episode.listened_seconds);
        assert!(episode.completed);
        assert_eq!(request.updated_at, episode.updated_at);
    }
}