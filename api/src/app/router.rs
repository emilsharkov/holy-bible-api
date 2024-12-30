use crate::{app, config, db, handlers};
use axum::{routing::get, Router};
use std::sync::Arc;
use std::error::Error;
use app::state::AppState;
use config::settings::Settings;
use handlers::bibles;
use db::{postgres, s3};

pub async fn get_app_router(settings: &Settings) -> Result<Router<AppState>, Box<dyn Error>> {
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
        .with_state(app_state);

    Ok(app_router)
}