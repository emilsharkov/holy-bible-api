use crate::config::postgres::PostgresConfig;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn get_client(config: &PostgresConfig) -> Result<Pool<Postgres>, Box<dyn Error>> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode={}",
        config.user, config.password, config.host, config.port, config.database, config.ssl_mode
    );
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}
