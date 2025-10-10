mod app;
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    utils::logging::init_logging();
    utils::openapi::try_generate_and_save_openapi_json_default();
    let settings = config::settings::Settings::new().unwrap();
    let router = app::router::get_app_router(&settings).await.unwrap();
    let listener = app::listener::bind_listener(&settings).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
