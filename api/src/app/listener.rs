use crate::config::settings::Settings;
use std::error::Error;

pub async fn bind_listener(settings: &Settings) -> Result<tokio::net::TcpListener, Box<dyn Error>> {
    let Settings {
        server_settings, ..
    } = settings;
    let address = format!("{}:{}", server_settings.host, server_settings.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    Ok(listener)
}
