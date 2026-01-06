use std::error::Error;

#[derive(Debug, Clone)]
pub struct MiddlewareConfig {
    pub timeout_seconds: u64,
    pub request_limit_per_hour: u16,
}

impl MiddlewareConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let timeout_seconds = std::env::var("TIMEOUT_SECONDS")
            .map_err(|_| "Missing required environment variable: TIMEOUT_SECONDS")?
            .parse()
            .map_err(|e| format!("Invalid TIMEOUT_SECONDS value: {}", e))?;
        let request_limit_per_hour = std::env::var("REQUEST_LIMIT_PER_HOUR")
            .map_err(|_| "Missing required environment variable: REQUEST_LIMIT_PER_HOUR")?
            .parse()
            .map_err(|e| format!("Invalid REQUEST_LIMIT_PER_HOUR value: {}", e))?;

        Ok(MiddlewareConfig {
            timeout_seconds,
            request_limit_per_hour,
        })
    }
}
