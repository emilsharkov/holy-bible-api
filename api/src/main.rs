mod app;
mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use config::settings;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new().unwrap();
    let app = app::app::get_app(&settings).await.unwrap();
    let addr = format!("{}:{}", settings.app_setting.host, settings.app_setting.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
