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
    let app_router = app::router::get_app_router(&settings).await.unwrap();
    let address = utils::address::get_app_address(&settings);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}
