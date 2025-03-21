use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::app::state::AppState;
use crate::handlers::{
    health::health::__path_get_health,
    bibles::bibles::__path_get_bibles,
    bibles::books::__path_get_bible_books,
    bibles::chapters::__path_get_bible_chapters,
    bibles::verses::__path_get_bible_verses,
    bibles::verses::__path_get_bible_verse_by_number,
    audio_bibles::audio_bibles::__path_get_audio_bibles,
    audio_bibles::books::__path_get_audio_bible_books,
    audio_bibles::chapters::__path_get_audio_bible_chapters,
    audio_bibles::chapters::__path_get_audio_chapter
};

#[derive(OpenApi)]
#[openapi(
    paths(
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
    )
)]
struct ApiDoc;

pub fn get_swagger_route() -> Router<AppState> {
    SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()).into()
}