use std::error::Error;

use dotenvy::dotenv;

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
pub struct CorsSettings {
    pub allow_any_origin: bool,
    pub allowed_origins: Vec<String>,
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
    pub cors_settings: CorsSettings,
    pub server_settings: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let database_settings = Self::get_database_settings()
            .map_err(|e| format!("Failed to load database settings: {}", e))?;
        let aws_settings =
            Self::get_aws_settings().map_err(|e| format!("Failed to load AWS settings: {}", e))?;
        let middleware_settings = Self::get_middleware_settings()
            .map_err(|e| format!("Failed to load middleware settings: {}", e))?;
        let redis_settings = Self::get_redis_settings()
            .map_err(|e| format!("Failed to load Redis settings: {}", e))?;
        let server_settings = Self::get_server_settings()
            .map_err(|e| format!("Failed to load server settings: {}", e))?;
        let cors_settings = Self::get_cors_settings()
            .map_err(|e| format!("Failed to load CORS settings: {}", e))?;

        Ok(Self {
            database_settings,
            aws_settings,
            middleware_settings,
            redis_settings,
            cors_settings,
            server_settings,
        })
    }

    fn get_database_settings() -> Result<DatabaseSettings, Box<dyn Error>> {
        let host = std::env::var("DB_HOST")
            .map_err(|_| "Missing required environment variable: DB_HOST")?;
        let port = std::env::var("DB_PORT")
            .map_err(|_| "Missing required environment variable: DB_PORT")?
            .parse()
            .map_err(|e| format!("Invalid DB_PORT value: {}", e))?;
        let database = std::env::var("DB_NAME")
            .map_err(|_| "Missing required environment variable: DB_NAME")?;
        let user = std::env::var("DB_USER")
            .map_err(|_| "Missing required environment variable: DB_USER")?;
        let password = std::env::var("DB_PASSWORD")
            .map_err(|_| "Missing required environment variable: DB_PASSWORD")?;
        let ssl_mode = std::env::var("DB_SSL_MODE")
            .map_err(|_| "Missing required environment variable: DB_SSL_MODE")?;

        Ok(DatabaseSettings {
            host,
            port,
            database,
            user,
            password,
            ssl_mode,
        })
    }

    fn get_aws_settings() -> Result<AwsSettings, Box<dyn Error>> {
        let access_key_id = std::env::var("S3_AWS_ACCESS_KEY_ID")
            .map_err(|_| "Missing required environment variable: S3_AWS_ACCESS_KEY_ID")?;
        let secret_access_key = std::env::var("S3_AWS_SECRET_ACCESS_KEY")
            .map_err(|_| "Missing required environment variable: S3_AWS_SECRET_ACCESS_KEY")?;
        let audio_bibles_bucket = std::env::var("AUDIO_BIBLES_BUCKET")
            .map_err(|_| "Missing required environment variable: AUDIO_BIBLES_BUCKET")?;
        let audio_bibles_bucket_aws_region = std::env::var("AUDIO_BIBLES_BUCKET_AWS_REGION")
            .map_err(|_| "Missing required environment variable: AUDIO_BIBLES_BUCKET_AWS_REGION")?;

        Ok(AwsSettings {
            access_key_id,
            secret_access_key,
            audio_bibles_bucket,
            audio_bibles_bucket_aws_region,
        })
    }

    pub fn get_middleware_settings() -> Result<MiddlewareSettings, Box<dyn Error>> {
        let timeout_seconds = std::env::var("TIMEOUT_SECONDS")
            .map_err(|_| "Missing required environment variable: TIMEOUT_SECONDS")?
            .parse()
            .map_err(|e| format!("Invalid TIMEOUT_SECONDS value: {}", e))?;
        let request_limit_per_hour = std::env::var("REQUEST_LIMIT_PER_HOUR")
            .map_err(|_| "Missing required environment variable: REQUEST_LIMIT_PER_HOUR")?
            .parse()
            .map_err(|e| format!("Invalid REQUEST_LIMIT_PER_HOUR value: {}", e))?;

        Ok(MiddlewareSettings {
            timeout_seconds,
            request_limit_per_hour,
        })
    }

    pub fn get_cors_settings() -> Result<CorsSettings, Box<dyn Error>> {
        let allow_any_origin = std::env::var("CORS_ALLOW_ANY_ORIGIN")
            .ok()
            .map(|value| value.parse::<bool>().unwrap_or(true))
            .unwrap_or(true);

        let allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();

        Ok(CorsSettings {
            allow_any_origin,
            allowed_origins,
        })
    }

    pub fn get_redis_settings() -> Result<RedisSettings, Box<dyn Error>> {
        let host = std::env::var("REDIS_HOST")
            .map_err(|_| "Missing required environment variable: REDIS_HOST")?;
        let port = std::env::var("REDIS_PORT")
            .map_err(|_| "Missing required environment variable: REDIS_PORT")?
            .parse()
            .map_err(|e| format!("Invalid REDIS_PORT value: {}", e))?;
        let password = std::env::var("REDIS_PASSWORD")
            .map_err(|_| "Missing required environment variable: REDIS_PASSWORD")?;

        Ok(RedisSettings {
            host,
            port,
            password,
        })
    }

    pub fn get_server_settings() -> Result<ServerSettings, Box<dyn Error>> {
        let host = std::env::var("SERVER_HOST")
            .map_err(|_| "Missing required environment variable: SERVER_HOST")?;
        let port = std::env::var("SERVER_PORT")
            .map_err(|_| "Missing required environment variable: SERVER_PORT")?
            .parse()
            .map_err(|e| format!("Invalid SERVER_PORT value: {}", e))?;

        Ok(ServerSettings { host, port })
    }
}
