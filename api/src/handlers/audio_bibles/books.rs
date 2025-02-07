use axum::{extract::{Path, State}, Json};

use crate::{app::state::AppState, db::s3::S3Client, models::{http::params::audio_bibles::books::BookPathParams, sql::audio_bibles}};

pub async fn get_books(
    State(app_state): State<AppState>,
    Path(params): Path<BookPathParams>
) -> Result<Json<()>, axum::response::Response> {
    let db_client = &*app_state.db_client;
    let S3Client { client, audio_bibles_bucket } = &*app_state.s3_client;

    let rows: Vec<audio_bibles::AudioBible> = sqlx::query_as(
        "SELECT audio_bible_id, language, version FROM verses WHERE bible_id = $1"
    )
        .bind(params.audio_bible_id)
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;

    let audio_bible_language = &rows
        .into_iter()
        .next()
        .ok_or_else(|| {
            axum::response::Response::builder()
                .status(404)
                .body("Verse not found".into())
                .expect("axum response builder failed")
        })?
        .language;

    let prefix = format!("languages/{}", audio_bible_language);
    let languages: Vec<String>  = client
        .list_objects_v2()
        .bucket(audio_bibles_bucket)
        .prefix(prefix)
        .send()
        .await
        .map_err(|err| {
            eprintln!("Detailed S3 request error: {:#?}", err);
            axum::response::Response::builder()
                .status(500)
                .body(format!("S3 request failed: {}", err).into())
                .expect("axum response builder failed")
        })?
        .contents
        .unwrap_or_default()
        .into_iter()
        .filter_map(|obj| obj.key)
        .collect();

        println!("{:?}", languages);

    Ok(Json(()))
}