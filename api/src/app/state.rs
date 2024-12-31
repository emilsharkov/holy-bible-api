use std::{error::Error, sync::Arc};
use aws_sdk_s3::Client;
use crate::{config,db};
use config::settings::Settings;
use db::{postgres,s3};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<sqlx::PgPool>,
    pub s3_client: Arc<Client>
}

impl AppState {
    pub async fn get_app_state(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let db_pool = postgres::pool::get_pool(&settings.database_settings).await?;
        let s3_client = s3::client::get_client(&settings.aws_settings).await?;
        let app_state = AppState {
            db_pool: Arc::new(db_pool),
            s3_client: Arc::new(s3_client),
        };
        Ok(app_state)
    }
}

