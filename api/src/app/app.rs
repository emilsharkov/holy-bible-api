use std::sync::Arc;
use std::error::Error;
use axum::{routing::get, Router};
use crate::{app, config, db};
use app::state::AppState;
use config::settings::Settings;

pub async fn get_app(settings: &Settings) -> Result<Router, Box<dyn Error>> {
    let db_pool = db::connection::establish_connection(&settings.database_settings).await?;
    let app_state = AppState {
        db_pool: Arc::new(db_pool),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(app_state);

    Ok(app)
}