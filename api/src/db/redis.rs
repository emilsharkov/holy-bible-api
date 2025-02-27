use redis::Client;
use crate::config::settings::RedisSettings;
use std::error::Error;
use tracing::info;

pub async fn get_client(settings: &RedisSettings) -> Result<Client, Box<dyn Error>> {
    let redis_url = format!(
        "redis://{}:{}",
        settings.host, settings.port
    );
    let client = Client::open(redis_url)?;
    info!("Connected to redis");
    Ok(client)
}