use crate::config::settings::DatabaseSettings;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn get_client(settings: &DatabaseSettings) -> Result<Pool<Postgres>, Box<dyn Error>> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode={}",
        settings.user,
        settings.password,
        settings.host,
        settings.port,
        settings.database,
        settings.ssl_mode
    );
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}
