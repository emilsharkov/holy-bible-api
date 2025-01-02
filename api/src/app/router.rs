use crate::{app,config,handlers,middleware};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use tower::ServiceBuilder;
use tracing::error;
use std::error::Error;
use app::state::AppState;
use config::settings::Settings;
use handlers::{bibles,books,chapters,verses};

pub async fn get_app_router(settings: &Settings) -> Result<Router, Box<dyn Error>> {
    let app_state = AppState::get_app_state(settings).await?;
    let app_router = Router::new()
        .route(
            "/bibles", 
            get(bibles::get_bibles)
        )
        .route(
            "/bibles/{bibleId}/search",
            get(bibles::get_verse_by_search))
        .route(
            "/bibles/{bibleId}/books", 
            get(books::get_books))
        .route(
            "/bibles/{bibleId}/books/{bookNum}/chapters", 
            get(chapters::get_chapters))
        .route(
            "/bibles/{bibleId}/books/{bookNum}/chapters/{chapterNum}/verses",
            get(verses::get_verses),
        )
        .route(
            "/bibles/{bibleId}/books/{bookNum}/chapters/{chapterNum}/verses/{verseNum}",
            get(verses::get_verse_by_number),
        )
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