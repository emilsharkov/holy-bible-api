use crate::models::http::response::bibles::bibles::Bible;
use crate::models::sql;
use sqlx::{PgPool, QueryBuilder};
use std::error::Error;

pub async fn get_bibles(
    db_client: &PgPool,
    language: Option<String>,
    version: Option<String>,
) -> Result<Vec<Bible>, Box<dyn Error>> {
    let mut query_builder = QueryBuilder::new("SELECT bible_id, language, version FROM bibles");

    let mut has_conditions = false;
    if language.is_some() || version.is_some() {
        query_builder.push(" WHERE ");
    }

    if let Some(lang) = &language {
        query_builder.push("language = ").push_bind(lang);
        has_conditions = true;
    }

    if let Some(ver) = &version {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("version = ").push_bind(ver);
    }

    let rows: Vec<sql::bible::Bible> = query_builder
        .build_query_as::<sql::bible::Bible>()
        .fetch_all(db_client)
        .await?;

    let bibles = rows
        .into_iter()
        .map(|bible| -> Bible {
            Bible {
                bible_id: bible.bible_id,
                language: bible.language,
                version: bible.version,
            }
        })
        .collect::<Vec<Bible>>();

    Ok(bibles)
}
