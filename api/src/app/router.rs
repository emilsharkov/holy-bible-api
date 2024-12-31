use crate::{app, config, handlers, middleware::{self, rate_limiter}};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tracing::error;
use std::{error::Error, time::Duration};
use app::state::AppState;
use config::settings::Settings;
use handlers::bibles;

pub async fn get_app_router(settings: &Settings) -> Result<Router, Box<dyn Error>> {
    let service = ServiceBuilder::new()
     .layer(TimeoutLayer::new(Duration::from_secs(30)));
    let app_state = AppState::get_app_state(settings).await?;
    let app_router = Router::new()
        .route("/bibles", get(bibles::get_bibles))
        // .route("/bibles/:bibleId/search", get(search_verse))
        // .route("/bibles/:bibleId/books", get(get_books))
        // .route("/bibles/:bibleId/books/:bookNum", get(get_book_by_number))
        // .route("/bibles/:bibleId/books/:bookNum/chapters", get(get_chapters))
        // .route(
        //     "/bibles/:bibleId/books/:bookNum/chapters/:chapterNum",
        //     get(get_chapter_by_number),
        // )
        // .route(
        //     "/bibles/:bibleId/books/:bookNum/chapters/:chapterNum/verses",
        //     get(get_verses),
        // )
        // .route(
        //     "/bibles/:bibleId/books/:bookNum/chapters/:chapterNum/verses/:verseNum",
        //     get(get_verse_by_number),
        // )
        .with_state(app_state)
        .layer(TimeoutLayer::new(Duration::from_nanos(0)))  // Set it to `0` but still did not get REQUEST TIMEOUT status code

        .layer(
            ServiceBuilder::new()
                .layer(middleware::trace::get_trace_layer())
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    error!("some error");
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(rate_limiter::get_rate_limiter_layer(
                    settings.middleware_settings.rate_limiter_per_second,
                    settings.middleware_settings.rate_limiter_burst_size,
                ))
        );
        

    Ok(app_router)
}