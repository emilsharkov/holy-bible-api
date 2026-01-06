use std::error::Error;

use dotenvy::dotenv;

use crate::config::aws::AwsConfig;
use crate::config::cors::CorsConfig;
use crate::config::postgres::PostgresConfig;
use crate::config::middleware::MiddlewareConfig;
use crate::config::redis::RedisConfig;
use crate::config::server::ServerConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_config: PostgresConfig,
    pub aws_config: AwsConfig,
    pub middleware_config: MiddlewareConfig,
    pub redis_config: RedisConfig,
    pub cors_config: CorsConfig,
    pub server_config: ServerConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let database_config = PostgresConfig::from_env()
            .map_err(|e| format!("Failed to load database config: {}", e))?;
        let aws_config = AwsConfig::from_env()
            .map_err(|e| format!("Failed to load AWS config: {}", e))?;
        let middleware_config = MiddlewareConfig::from_env()
            .map_err(|e| format!("Failed to load middleware config: {}", e))?;
        let redis_config = RedisConfig::from_env()
            .map_err(|e| format!("Failed to load Redis config: {}", e))?;
        let server_config = ServerConfig::from_env()
            .map_err(|e| format!("Failed to load server config: {}", e))?;
        let cors_config = CorsConfig::from_env()
            .map_err(|e| format!("Failed to load CORS config: {}", e))?;

        Ok(Self {
            database_config,
            aws_config,
            middleware_config,
            redis_config,
            cors_config,
            server_config,
        })
    }
}
