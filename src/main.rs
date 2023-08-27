use std::net::SocketAddr;

use crate::app::create_app;
use crate::state::AppState;
use dotenvy::dotenv;

mod app;
mod database;
mod endpoints;
mod error_handling;
mod fixtures;
mod models;
mod schema;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    tracing::info!("loading .env file: {:?}", dotenv());

    let app = create_app(AppState::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "dimppl-server"
}
