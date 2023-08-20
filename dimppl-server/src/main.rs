use std::net::SocketAddr;

use axum::ServiceExt;
use dotenvy::dotenv;

use crate::app::create_app;

mod schema;
mod database;
mod endpoints;
mod models;
mod state;
mod app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let app = create_app();

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
