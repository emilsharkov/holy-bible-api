use axum::{
    body::Body,
    extract::{Path, Query, State},
    response::Response,
    routing::get,
    Json, Router,
};
use tokio_util::io::ReaderStream;

use crate::{
    app::state::AppState,
    controller::error::ServiceErrorExt,
    models::http::{
        params::audio_bibles::{
            audio_bible::AudioBibleQueryParams,
            books::BookPathParams,
            chapters::{AudioChapterPathParams, ChaptersPathParams},
        },
        response::audio_bibles::{
            audio_bibles::GetAudioBiblesRes, books::GetAudioBooksRes, chapters::GetAudioChaptersRes,
        },
    },
};

pub fn get_audio_bible_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/audio_bibles", get(get_audio_bibles))
        .route(
            "/audio_bibles/:audio_bible_id/books",
            get(get_audio_bible_books),
        )
        .route(
            "/audio_bibles/:audio_bible_id/books/:book_num/chapters",
            get(get_audio_bible_chapters),
        )
        .route(
            "/audio_bibles/:audio_bible_id/books/:book_num/chapters/:chapter_num",
            get(get_audio_chapter),
        )
}

#[utoipa::path(
    get,
    path = "/audio_bibles",
    params(AudioBibleQueryParams),
    responses(
        (status = 200, body = GetAudioBiblesRes)
    )
)]
pub async fn get_audio_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<AudioBibleQueryParams>,
) -> Result<Json<GetAudioBiblesRes>, axum::response::Response> {
    let audio_bible_service = &app_state.audio_bible_service;
    let language = params.language;
    let version = params.version;
    let audio_bibles = audio_bible_service
        .get_audio_bibles(language, version)
        .await
        .into_axum_response()?;
    Ok(Json(GetAudioBiblesRes { audio_bibles }))
}

#[utoipa::path(
    get,
    path = "/audio_bibles/{audio_bible_id}/books",
    params(BookPathParams),
    responses(
        (status = 200, body = GetAudioBooksRes)
    )
)]
pub async fn get_audio_bible_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPathParams>,
) -> Result<Json<GetAudioBooksRes>, axum::response::Response> {
    let audio_bible_service = &app_state.audio_bible_service;
    let num_books = audio_bible_service
        .get_audio_bible_books(params.audio_bible_id)
        .await
        .into_axum_response()?;
    Ok(Json(GetAudioBooksRes { num_books }))
}

#[utoipa::path(
    get,
    path = "/audio_bibles/{audio_bible_id}/books/{book_num}/chapters",
    params(ChaptersPathParams),
    responses(
        (status = 200, body = GetAudioChaptersRes)
    )
)]
pub async fn get_audio_bible_chapters(
    State(app_state): State<AppState>,
    Path(params): Path<ChaptersPathParams>,
) -> Result<Json<GetAudioChaptersRes>, axum::response::Response> {
    let audio_bible_service = &app_state.audio_bible_service;
    let num_chapters = audio_bible_service
        .get_audio_bible_chapters(params.audio_bible_id, params.book_num)
        .await
        .into_axum_response()?;
    Ok(Json(GetAudioChaptersRes { num_chapters }))
}

#[utoipa::path(
    get,
    path = "/audio_bibles/{audio_bible_id}/books/{book_num}/chapters/{chapter_num}",
    params(AudioChapterPathParams),
    responses(
        (status = 200, description = "Returns the audio chapter file", content_type = "audio/mpeg"),
        (status = 404, description = "Audio Chapter not found", body = String)
    )
)]
pub async fn get_audio_chapter(
    State(app_state): State<AppState>,
    Path(params): Path<AudioChapterPathParams>,
) -> Result<Response<Body>, axum::response::Response> {
    let audio_bible_service = &app_state.audio_bible_service;
    let object_output = audio_bible_service
        .get_audio_chapter(params.audio_bible_id, params.book_num, params.chapter_num)
        .await
        .into_axum_response()?;

    let async_buf_reader = object_output.body.into_async_read();
    let stream = ReaderStream::new(async_buf_reader);
    let body = Body::from_stream(stream);

    let response = Response::builder()
        .header("Content-Type", "audio/mpeg")
        .body(body)
        .unwrap();

    Ok(response)
}
