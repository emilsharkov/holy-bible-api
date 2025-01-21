use sqlx::{Pool, Postgres};
use crate::config::settings::DatabaseSettings;
use std::error::Error;

pub async fn get_client(settings: &DatabaseSettings) -> Result<Pool<Postgres>, Box<dyn Error>> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.user, settings.password, settings.host, settings.port, settings.database
    );
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}