use crate::{
    app, config, controller,
    middleware::{self, cors},
};
use app::state::AppState;
use axum::{
    error_handling::HandleErrorLayer,
    extract::connect_info::{ConnectInfo, IntoMakeServiceWithConnectInfo},
    http::StatusCode,
    middleware::from_fn_with_state,
    BoxError, Router,
};
use config::settings::Config;
use std::{error::Error, net::SocketAddr};
use tower::ServiceBuilder;
use tracing::error;

pub async fn get_app_router(
    config: &Config,
) -> Result<IntoMakeServiceWithConnectInfo<Router, SocketAddr>, Box<dyn Error>> {
    let app_config = config.clone();
    let app_state = AppState::get_app_state(&app_config).await?;
    let app_router = Router::new()
        .merge(controller::health::get_health_route())
        .merge(controller::bibles::get_bible_routes())
        .merge(controller::audio_bibles::get_audio_bible_routes())
        .merge(controller::swagger::get_swagger_route())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::trace::get_trace_layer())
                .layer(middleware::compression::get_compression_layer())
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    error!("Middleware Error: {}", err);
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(middleware::timeout::get_timeout_layer(
                    app_config.middleware_config.timeout_seconds,
                ))
                .layer(cors::get_cors_layer(&app_config.cors_config))
                .layer(from_fn_with_state(
                    app_state.clone(),
                    move |state, connect_info: ConnectInfo<SocketAddr>, req, next| {
                        middleware::rate_limiter::rate_limiter(
                            state,
                            connect_info,
                            req,
                            next,
                            app_config.middleware_config.request_limit_per_hour,
                        )
                    },
                )),
        )
        .with_state(app_state)
        .into_make_service_with_connect_info::<SocketAddr>();

    Ok(app_router)
}
