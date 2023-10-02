use crate::endpoints::create_device::create_device;
use crate::endpoints::create_podcast::create_podcast;
use crate::endpoints::create_user::create_user;
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

mod create_device;
pub mod create_podcast;
pub mod create_user;
pub mod websocket;

pub trait RouterExt {
    fn apply_app_routes(self) -> Self;
}

impl RouterExt for Router<AppState> {
    fn apply_app_routes(self) -> Self {
        self.route("/user", post(create_user))
            .route("/devices", post(create_device))
            .route("/podcasts", post(create_podcast))
            .route("/ws", get(websocket::websocket_handler))
            .route("/", get(root))
    }
}

async fn root() -> &'static str {
    "dimppl-server"
}
