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
    let settings = config::settings::Settings::new().unwrap();
    let router = app::router::get_app_router(&settings).await.unwrap();
    let listener = app::listener::bind_listener(&settings).await.unwrap();
    tracing::info!(
        "Server started on {}:{}",
        settings.server_settings.host,
        settings.server_settings.port
    );
    axum::serve(listener, router).await.unwrap();
}
