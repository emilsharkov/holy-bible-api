use crate::{app,config,middleware,routes};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use tower::ServiceBuilder;
use tracing::error;
use std::error::Error;
use app::state::AppState;
use config::settings::Settings;

pub async fn get_app_router(settings: &Settings) -> Result<Router, Box<dyn Error>> {
    let app_state = AppState::get_app_state(settings).await?;
    let app_router = Router::new()
        .merge(routes::bibles::get_bible_routes())
        .merge(routes::audio_bibles::get_audio_bible_routes())
        .merge(routes::swagger::get_swagger_route())
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    error!("Middleware Error: {}", err);
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(middleware::trace::get_trace_layer())
                .layer(middleware::compression::get_compression_layer())
                .layer(middleware::timeout::get_timeout_layer(
                    settings.middleware_settings.timeout_seconds,
                ))
        );

    Ok(app_router)
}