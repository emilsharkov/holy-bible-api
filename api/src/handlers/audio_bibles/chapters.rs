use aws_sdk_s3::error;
use axum::{body::Body, extract::{Path, State}, response::Response, Json};
use tokio_util::io::ReaderStream;

use crate::{app::state::AppState, db::s3::S3Client, models::{http::{params::audio_bibles::chapters::{AudioChapterPathParams, ChaptersPathParams}, response::audio_bibles::chapters::GetAudioChaptersRes}, sql::audio_bibles}};

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
) -> Result<Json<GetAudioChaptersRes>, Response> {
    let db_client = &*app_state.db_client;
    let rows: Vec<audio_bibles::Count> = sqlx::query_as(
    "SELECT COUNT(distinct chapter) 
        FROM audio_bible_chapters 
        WHERE audio_bible_id = $1
        AND book = $2"
    )
        .bind(params.audio_bible_id)
        .bind(params.book_num)
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;
    
    let num_chapters = rows
        .into_iter()
        .next()
        .ok_or_else(|| {
            Response::builder()
                .status(404)
                .body("Audio Bible not found".into())
                .expect("axum response builder failed")
        })?
        .count;

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
    Path(params): Path<AudioChapterPathParams>
) -> Result<Response<Body>, Response> {
    let db_client = &*app_state.db_client;
    let S3Client { client, audio_bibles_bucket } = &*app_state.s3_client;

    let rows: Vec<audio_bibles::AudioBible> = sqlx::query_as(
        "SELECT audio_bible_id, language, version FROM audio_bibles WHERE audio_bible_id = $1"
    )
        .bind(params.audio_bible_id)
        .fetch_all(db_client)
        .await
        .map_err(|_err| {
            Response::builder()
                .status(404)
                .body("audio_bible_id not found".into())
                .expect("axum response builder failed")
        })?;

    let audio_bible_language = rows
        .into_iter()
        .next()
        .ok_or_else(|| {
            Response::builder()
                .status(404)
                .body("Audio Bible not found".into())
                .expect("axum response builder failed")
        })?
        .language;

    let file_key = format!(
        "languages/{}/{}/{}.mp3", 
        audio_bible_language, 
        params.book_num, 
        params.chapter_num
    );

    let message = format!("{}",file_key);
    info!(message);

    let object_output = client
        .get_object()
        .bucket(audio_bibles_bucket)
        .key(file_key)
        .send()
        .await
        .map_err(|err| {
            error!("Failed to get object from S3: {}", err);
            Response::builder()
                .status(404)
                .body("Resource does not exist".into())
                .unwrap()
        })?;

    let async_buf_reader = object_output.body.into_async_read();
    let stream =  ReaderStream::new(async_buf_reader);
    let body = Body::from_stream(stream);

    let response = Response::builder()
        .header("Content-Type", "audio/mpeg")
        .body(body)
        .unwrap();

    Ok(response)
}