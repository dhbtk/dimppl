use axum::Router;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::endpoints::RouterExt;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .apply_app_routes()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}

#[cfg(test)]
pub fn create_test_app() -> (AppState, Router) {
    if std::env::var("DATABASE_URL").is_err() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/dimppl_test");
    }
    std::env::set_var("DIMPPL_TEST", "true");
    let state = AppState::new();

    (state.clone(), create_app(state))
}
