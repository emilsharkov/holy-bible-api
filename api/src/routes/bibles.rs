use axum::{
    routing::get,
    Router,
};

use crate::{app::state::AppState, handlers::{bibles, books, chapters, verses}};

pub fn get_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route(
            "/bibles", 
        get(bibles::get_bibles)
    )
        .route(
            "/bibles/:bibleId/search", 
            get(bibles::get_verse_by_search)
        )
        .route(
            "/bibles/:bibleId/books", 
            get(books::get_books)
        )
        .route(
            "/bibles/:bibleId/books/:bookNum/chapters",
            get(chapters::get_chapters)
        )
        .route(
            "/bibles/:bibleId/books/:bookNum/chapters/:chapterNum/verses",
            get(verses::get_verses),
        )
        .route(
            "/bibles/:bibleId/books/:bookNum/chapters/:chapterNum/verses/:verseNum",
            get(verses::get_verse_by_number),
        )
}