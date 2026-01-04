use crate::app::state::AppState;
use crate::controller::{
    audio_bibles::{
        __path_get_audio_bible_books, __path_get_audio_bible_chapters, __path_get_audio_bibles,
        __path_get_audio_chapter,
    },
    bibles::{
        __path_get_bible_books, __path_get_bible_chapters, __path_get_bible_verse_by_number,
        __path_get_bible_verses, __path_get_bibles,
    },
    health::__path_get_health,
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    get_health,
    get_bibles,
    get_bible_books,
    get_bible_chapters,
    get_bible_verses,
    get_bible_verse_by_number,
    get_audio_bibles,
    get_audio_bible_books,
    get_audio_bible_chapters,
    get_audio_chapter
))]
pub struct ApiDoc;

pub fn get_swagger_route() -> Router<AppState> {
    SwaggerUi::new("/docs")
        .url("/api-doc/openapi.json", ApiDoc::openapi())
        .into()
}
