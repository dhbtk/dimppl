use crate::endpoints::create_device::create_device;
use crate::endpoints::create_podcast::create_podcast;
use crate::endpoints::create_user::create_user;
use crate::endpoints::sync_state::sync_state;
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use crate::endpoints::submit_progress::submit_progress;

mod create_device;
pub mod create_podcast;
pub mod create_user;
mod sync_state;
pub mod websocket;
pub mod submit_progress;

pub trait RouterExt {
    fn apply_app_routes(self) -> Self;
}

impl RouterExt for Router<AppState> {
    fn apply_app_routes(self) -> Self {
        self.route("/user", post(create_user))
            .route("/devices", post(create_device))
            .route("/podcasts", post(create_podcast))
            .route("/ws", get(websocket::websocket_handler))
            .route("/sync", post(sync_state))
            .route("/submit_progress", post(submit_progress))
            .route("/", get(root))
    }
}

async fn root() -> &'static str {
    "dimppl-server"
}
