use crate::{app, config, db, handlers};
use axum::{routing::get, Router};
use std::sync::Arc;
use std::error::Error;
use app::state::AppState;
use config::settings::Settings;
use handlers::{bibles};

pub async fn get_app(settings: &Settings) -> Result<Router, Box<dyn Error>> {
    let app_state = get_app_state(settings).await?;
    let app = Router::new()
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

    Ok(app)
}

async fn get_app_state(settings: &Settings) -> Result<AppState, Box<dyn Error>> {
    let db_pool = db::connection::establish_connection(&settings.database_settings).await?;
    let app_state = AppState {
        db_pool: Arc::new(db_pool),
    };
    Ok(app_state)
}