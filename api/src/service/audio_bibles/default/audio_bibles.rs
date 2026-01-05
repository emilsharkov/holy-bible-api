use std::error::Error;

use crate::models::http::response::audio_bibles::audio_bibles::AudioBible;
use crate::models::sql;
use sqlx::{PgPool, QueryBuilder};

pub async fn get_audio_bibles(
    db_client: &PgPool,
    language: Option<String>,
    version: Option<String>,
) -> Result<Vec<AudioBible>, Box<dyn Error>> {
    let mut query_builder =
        QueryBuilder::new("SELECT audio_bible_id, language, version FROM audio_bibles");

    let mut has_conditions = false;
    if language.is_some() || version.is_some() {
        query_builder.push(" WHERE ");
    }

    if let Some(language) = &language {
        query_builder.push("language = ").push_bind(language);
        has_conditions = true;
    }

    if let Some(version) = &version {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("version = ").push_bind(version);
    }

    let rows: Vec<sql::audio_bibles::AudioBible> = query_builder
        .build_query_as::<sql::audio_bibles::AudioBible>()
        .fetch_all(db_client)
        .await?;

    let audio_bibles = rows
        .into_iter()
        .map(|audio_bible| -> AudioBible {
            AudioBible {
                audio_bible_id: audio_bible.audio_bible_id,
                language: audio_bible.language,
                version: audio_bible.version,
            }
        })
        .collect::<Vec<AudioBible>>();

    Ok(audio_bibles)
}

