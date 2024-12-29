mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod state;
mod utils;

use std::sync::Arc;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    config::env::load_env();
    utils::logging::init_logging();

    let db_pool = db::connection::establish_connection().await;
    let app_state = state::app_state::AppState {
        db_pool: Arc::new(db_pool),
    };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
