use crate::database::Pool;
use crate::error_handling::AppResult;
use crate::models::{User, UserDevice};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::headers::HeaderMap;
use axum::response::IntoResponse;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(pool): State<Pool>,
) -> AppResult<impl IntoResponse> {
    let mut conn = pool.get().await?;
    let (user, device) =
        crate::models::user_device::user_and_device_from_http_request(&headers, &mut conn).await?;
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, user, device)))
}

async fn handle_socket(mut socket: WebSocket, user: User, device: UserDevice) {
    socket
        .send(Message::Text(format!(
            "User {} connected, device {}",
            user.access_key, device.name
        )))
        .await
        .unwrap();
    // TODO: loop
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
}
