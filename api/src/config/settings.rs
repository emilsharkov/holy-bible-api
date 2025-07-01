use std::error::Error;
use crate::config;

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub ssl_mode: String,
}

#[derive(Debug, Clone)]
pub struct AwsSettings {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub audio_bibles_bucket: String,
    pub audio_bibles_bucket_aws_region: String,
}

#[derive(Debug, Clone)]
pub struct MiddlewareSettings {
    pub timeout_seconds: u64,
    pub request_limit_per_hour: u16,
}

#[derive(Debug, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub aws_settings: AwsSettings,
    pub middleware_settings: MiddlewareSettings,
    pub redis_settings: RedisSettings,
    pub server_settings: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        config::env::load_env();
        let database_settings = Self::get_database_settings()?;
        let aws_settings = Self::get_aws_settings()?;
        let middleware_settings = Self::get_middleware_settings()?;
        let redis_settings = Self::get_redis_settings()?;
        let server_settings = Self::get_server_settings()?;

        Ok(Self {
            database_settings,
            aws_settings,
            middleware_settings,
            redis_settings,
            server_settings,
        })
    }

    fn get_database_settings() -> Result<DatabaseSettings, Box<dyn Error>> {
        let database_settings = DatabaseSettings {
            host: std::env::var("DB_HOST")?,
            port: std::env::var("DB_PORT")?.parse()?,
            database: std::env::var("DB_NAME")?,
            user: std::env::var("DB_USER")?,
            password: std::env::var("DB_PASSWORD")?,
            ssl_mode: std::env::var("DB_SSL_MODE")?,
        };
        Ok(database_settings)
    }
    
    fn get_aws_settings() -> Result<AwsSettings, Box<dyn Error>> {
        let aws_settings = AwsSettings {
            access_key_id: std::env::var("S3_AWS_ACCESS_KEY_ID")?,
            secret_access_key: std::env::var("S3_AWS_SECRET_ACCESS_KEY")?,
            audio_bibles_bucket: std::env::var("AUDIO_BIBLES_BUCKET")?,
            audio_bibles_bucket_aws_region: std::env::var("AUDIO_BIBLES_BUCKET_AWS_REGION")?,
        };
        Ok(aws_settings)
    }

    pub fn get_middleware_settings() -> Result<MiddlewareSettings, Box<dyn Error>> {
        let middleware_settings = MiddlewareSettings {
            timeout_seconds: std::env::var("TIMEOUT_SECONDS")?.parse()?,
            request_limit_per_hour: std::env::var("REQUEST_LIMIT_PER_HOUR")?.parse()?,
        };
        Ok(middleware_settings)
    }

    pub fn get_redis_settings() -> Result<RedisSettings, Box<dyn Error>> {
        let redis_settings = RedisSettings {
            host: std::env::var("REDIS_HOST")?,
            port: std::env::var("REDIS_PORT")?.parse()?,
            password: std::env::var("REDIS_PASSWORD")?,
        };
        Ok(redis_settings)
    }

    pub fn get_server_settings() -> Result<ServerSettings, Box<dyn Error>> {
        let server_settings = ServerSettings {
            host: std::env::var("SERVER_HOST")?,
            port: std::env::var("SERVER_PORT")?.parse()?,
        };
        Ok(server_settings)
    }
}