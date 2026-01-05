use crate::config::redis::RedisConfig;
use redis::Client;
use std::error::Error;

pub async fn get_client(config: &RedisConfig) -> Result<Client, Box<dyn Error>> {
    let redis_url = format!(
        "redis://:{}@{}:{}",
        config.password, config.host, config.port
    );
    let client = Client::open(redis_url)?;
    Ok(client)
}
