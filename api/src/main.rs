mod app;
mod config;
mod controller;
mod db;
mod middleware;
mod models;
mod repo;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    utils::logging::init_logging();
    utils::openapi::try_generate_and_save_openapi_json_default();
    let config = config::settings::Config::new().unwrap();
    let router = app::router::get_app_router(&config).await.unwrap();
    let listener = app::listener::bind_listener(&config).await.unwrap();
    tracing::info!(
        "Server started on {}:{}",
        config.server_config.host,
        config.server_config.port
    );
    axum::serve(listener, router).await.unwrap();
}
