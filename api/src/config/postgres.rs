use std::error::Error;

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub ssl_mode: String,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
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

        Ok(PostgresConfig {
            host,
            port,
            database,
            user,
            password,
            ssl_mode,
        })
    }
}

