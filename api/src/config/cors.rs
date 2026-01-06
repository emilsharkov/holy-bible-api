use std::error::Error;

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allow_any_origin: bool,
    pub allowed_origins: Vec<String>,
}

impl CorsConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
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

        Ok(CorsConfig {
            allow_any_origin,
            allowed_origins,
        })
    }
}
