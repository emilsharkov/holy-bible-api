use axum::{
    routing::get,
    Router,
};

use crate::{app::state::AppState, handlers::audio_bibles::{audio_bibles, books, chapters}};

pub fn get_audio_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route(
            "/audio_bibles", 
            get(audio_bibles::get_audio_bibles)
        )
        .route(
            "/audio_bibles/:audio_bible_id/books", 
            get(books::get_books)
        )
        .route(
            "/audio_bibles/:audio_bible_id/books/:book_num/chapters", 
            get(chapters::get_chapters)
        )
        .route(
            "/audio_bibles/:audio_bible_id/books/:book_num/chapters/:chapter_num", 
            get(chapters::get_audio_chapter)
        )
}
