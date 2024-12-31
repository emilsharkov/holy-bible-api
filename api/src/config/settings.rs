use std::error::Error;
use crate::config;

#[derive(Debug)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct AwsSettings {
    pub access_key_id: String,
    pub secret_access_key: String,
}

#[derive(Debug)]
pub struct MiddlewareSettings {
    pub rate_limiter_per_second: u64,
    pub rate_limiter_burst_size: u32,
    pub timeout_seconds: u64
}

#[derive(Debug)]
pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub app_setting: AppSettings,
    pub aws_settings: AwsSettings,
    pub middleware_settings: MiddlewareSettings,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        config::env::load_env();
        let database_settings = Self::get_database_settings()?;
        let app_setting = Self::get_app_settings()?;
        let aws_settings = Self::get_aws_settings()?;
        let middleware_settings = Self::get_middleware_settings()?;

        Ok(Self {
            database_settings,
            app_setting,
            aws_settings,
            middleware_settings,
        })
    }

    fn get_database_settings() -> Result<DatabaseSettings, Box<dyn Error>> {
        let database_settings = DatabaseSettings {
            host: std::env::var("DB_HOST")?,
            port: std::env::var("DB_PORT")?.parse()?,
            database: std::env::var("DB_NAME")?,
            user: std::env::var("DB_USER")?,
            password: std::env::var("DB_PASSWORD")?,
        };
        Ok(database_settings)
    }
    
    fn get_app_settings() -> Result<AppSettings, Box<dyn Error>> {
        let app_settings = AppSettings {
            host: std::env::var("APP_HOST")?,
            port: std::env::var("APP_PORT")?.parse()?,
        };
        Ok(app_settings)
    }

    fn get_aws_settings() -> Result<AwsSettings, Box<dyn Error>> {
        let aws_settings = AwsSettings {
            access_key_id: std::env::var("AWS_ACCESS_KEY_ID")?,
            secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")?,
        };
        Ok(aws_settings)
    }

    pub fn get_middleware_settings() -> Result<MiddlewareSettings, Box<dyn Error>> {
        let middleware_settings = MiddlewareSettings {
            rate_limiter_per_second: std::env::var("RATE_LIMITER_PER_SECOND")?.parse()?,
            rate_limiter_burst_size: std::env::var("RATE_LIMITER_BURST_SIZE")?.parse()?,
            timeout_seconds: std::env::var("TIMEOUT_SECONDS")?.parse()?,
        };
        Ok(middleware_settings)
    }
}