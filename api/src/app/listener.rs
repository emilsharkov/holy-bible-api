use crate::config::settings::Config;
use std::error::Error;

pub async fn bind_listener(config: &Config) -> Result<tokio::net::TcpListener, Box<dyn Error>> {
    let Config { server_config, .. } = config;
    let address = format!("{}:{}", server_config.host, server_config.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    Ok(listener)
}
