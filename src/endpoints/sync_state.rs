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
    Ok(Json(SyncStateResponse::default()))
}
