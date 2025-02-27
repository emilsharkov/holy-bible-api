use axum::extract::{Query, State};
use axum::Json;
use sqlx::{QueryBuilder, PgPool};
use crate::app::state::AppState;
use crate::models::http::params::audio_bibles::audio_bible::AudioBibleQueryParams;
use crate::models::http::response::audio_bibles::audio_bibles::{GetAudioBiblesRes, AudioBible};
use crate::models::sql;

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
    Query(params): Query<AudioBibleQueryParams>
) -> Result<Json<GetAudioBiblesRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let mut query_builder = QueryBuilder::new("SELECT audio_bible_id, language, version FROM audio_bibles");
    
    let mut has_conditions = false;
    if params.language.is_some() || params.version.is_some() {
        query_builder.push(" WHERE ");
    }

    if let Some(language) = &params.language {
        query_builder.push("language = ").push_bind(language);
        has_conditions = true;
    }

    if let Some(version) = &params.version {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("version = ").push_bind(version);
    }

    let rows: Vec<sql::audio_bibles::AudioBible> = query_builder.build_query_as::<sql::audio_bibles::AudioBible>()
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;

    let audio_bibles = rows
        .into_iter()
        .map(|audio_bible| -> AudioBible { AudioBible {
            audio_bible_id: audio_bible.audio_bible_id,
            language: audio_bible.language,
            version: audio_bible.version
        }})
        .collect::<Vec<AudioBible>>();

    Ok(Json(GetAudioBiblesRes { audio_bibles }))
}