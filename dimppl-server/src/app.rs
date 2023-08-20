use axum::Router;
use axum::routing::{get, post};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use crate::endpoints::create_user::create_user;
use crate::root;
use crate::state::AppState;
use dotenvy::dotenv;


pub fn create_app() -> Router {
    tracing::info!("loading .env file: {:?}", dotenv());

    Router::new()
        .route("/user", post(create_user))
        .route("/", get(root))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)))
        .with_state(AppState::new())
}

pub fn create_test_app() -> Router {
    if std::env::var("DATABASE_URL").is_err() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/dimppl_test");
        std::env::set_var("DIMPPL_TEST", "true");
    }

    create_app()
}
