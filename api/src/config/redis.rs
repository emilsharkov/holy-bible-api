use std::error::Error;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let host = std::env::var("REDIS_HOST")
            .map_err(|_| "Missing required environment variable: REDIS_HOST")?;
        let port = std::env::var("REDIS_PORT")
            .map_err(|_| "Missing required environment variable: REDIS_PORT")?
            .parse()
            .map_err(|e| format!("Invalid REDIS_PORT value: {}", e))?;
        let password = std::env::var("REDIS_PASSWORD")
            .map_err(|_| "Missing required environment variable: REDIS_PASSWORD")?;

        Ok(RedisConfig {
            host,
            port,
            password,
        })
    }
}

