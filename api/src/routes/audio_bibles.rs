use axum::{
    routing::get,
    Router,
};

use crate::app::state::AppState;

pub fn get_audio_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/audio_bibles", get(|| async { Ok::<_, axum::response::ErrorResponse>("Hello, audio bibles!") }))
}
