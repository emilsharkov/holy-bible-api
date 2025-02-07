mod app;
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use lambda_http::run;

#[tokio::main]
async fn main() {
    utils::logging::init_logging();
    let settings = config::settings::Settings::new().unwrap();
    let router = app::router::get_app_router(&settings).await.unwrap();
    run(router).await.unwrap();
}
