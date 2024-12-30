mod app;
mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    utils::logging::init_logging();
    let settings = config::settings::Settings::new().unwrap();
    let app = app::app::get_app(&settings).await.unwrap();
    let addr = format!("{}:{}", settings.app_setting.host, settings.app_setting.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
