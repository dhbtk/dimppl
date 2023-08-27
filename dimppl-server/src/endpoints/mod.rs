use axum::Router;
use axum::routing::{get, post};
use crate::endpoints::create_device::create_device;
use crate::endpoints::create_user::create_user;
use crate::root;
use crate::state::AppState;

pub mod create_user;
mod create_device;

pub trait RouterExt {
    fn apply_app_routes(self) -> Self;
}

impl RouterExt for Router<AppState> {
    fn apply_app_routes(self) -> Self {
        self.route("/user", post(create_user))
            .route("/devices", post(create_device))
            .route("/", get(root))
    }
}
