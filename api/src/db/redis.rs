use redis::Client;
use crate::config::settings::RedisSettings;
use std::error::Error;

pub async fn get_client(settings: &RedisSettings) -> Result<Client, Box<dyn Error>> {
    let redis_url = format!(
        "redis://:{}@{}:{}",
        settings.password, settings.host, settings.port
    );
    let client = Client::open(redis_url)?;
    Ok(client)
}