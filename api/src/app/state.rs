use std::{error::Error, sync::Arc};
use crate::{config,db};
use config::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<sqlx::PgPool>,
    pub s3_client: Arc<aws_sdk_s3::Client>,
    pub redis_client: Arc<redis::Client>,
}

impl AppState {
    pub async fn get_app_state(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let db_client = db::postgres::get_client(&settings.database_settings).await?;
        let s3_client = db::s3::get_client(&settings.aws_settings).await?;
        let redis_client = db::redis::get_client(&settings.redis_settings).await?;
        let app_state = AppState {
            db_client: Arc::new(db_client),
            s3_client: Arc::new(s3_client),
            redis_client: Arc::new(redis_client),
        };
        Ok(app_state)
    }
}

