use axum::{routing::get, Router};

use crate::{
    app::state::AppState,
    handlers::bibles::{bibles, books, chapters, verses},
};

pub fn get_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/bibles", get(bibles::get_bibles))
        .route("/bibles/:bible_id/books", get(books::get_bible_books))
        .route(
            "/bibles/:bible_id/books/:book_num/chapters",
            get(chapters::get_bible_chapters),
        )
        .route(
            "/bibles/:bible_id/books/:book_num/chapters/:chapter_num/verses",
            get(verses::get_bible_verses),
        )
        .route(
            "/bibles/:bible_id/books/:book_num/chapters/:chapter_num/verses/:verse_num",
            get(verses::get_bible_verse_by_number),
        )
}
