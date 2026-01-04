use crate::{
    app, config,
    middleware::{self, cors},
    routes,
};
use app::state::AppState;
use axum::{
    error_handling::HandleErrorLayer,
    extract::connect_info::{ConnectInfo, IntoMakeServiceWithConnectInfo},
    http::StatusCode,
    middleware::from_fn_with_state,
    BoxError, Router,
};
use config::settings::Settings;
use std::{error::Error, net::SocketAddr};
use tower::ServiceBuilder;
use tracing::error;

pub async fn get_app_router(
    settings: &Settings,
) -> Result<IntoMakeServiceWithConnectInfo<Router, SocketAddr>, Box<dyn Error>> {
    let app_settings = settings.clone();
    let app_state = AppState::get_app_state(&app_settings).await?;
    let app_router = Router::new()
        .merge(routes::health::get_health_route())
        .merge(routes::bibles::get_bible_routes())
        .merge(routes::audio_bibles::get_audio_bible_routes())
        .merge(routes::swagger::get_swagger_route())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::trace::get_trace_layer())
                .layer(middleware::compression::get_compression_layer())
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    error!("Middleware Error: {}", err);
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(middleware::timeout::get_timeout_layer(
                    app_settings.middleware_settings.timeout_seconds,
                ))
                .layer(cors::get_cors_layer(&app_settings.cors_settings))
                .layer(from_fn_with_state(
                    app_state.clone(),
                    move |state, connect_info: ConnectInfo<SocketAddr>, req, next| {
                        middleware::rate_limiter::rate_limiter(
                            state,
                            connect_info,
                            req,
                            next,
                            app_settings.middleware_settings.request_limit_per_hour,
                        )
                    },
                )),
        )
        .with_state(app_state)
        .into_make_service_with_connect_info::<SocketAddr>();

    Ok(app_router)
}
