use std::error::Error;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let host = std::env::var("SERVER_HOST")
            .map_err(|_| "Missing required environment variable: SERVER_HOST")?;
        let port = std::env::var("SERVER_PORT")
            .map_err(|_| "Missing required environment variable: SERVER_PORT")?
            .parse()
            .map_err(|e| format!("Invalid SERVER_PORT value: {}", e))?;

        Ok(ServerConfig { host, port })
    }
}

