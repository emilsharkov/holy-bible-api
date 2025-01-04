use axum::extract::{Path, Query, State};
use axum::Json;
use sqlx::{QueryBuilder, PgPool};
use crate::app::state::AppState;
use crate::models::http::params::bible::{BibleQueryParams, BibleSearchQueryParams};
use crate::models::http::response::verses::{GetVersesRes, Verse};
use crate::models::sql::bible;

pub async fn get_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<BibleQueryParams>
) -> Result<String, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let mut query_builder = QueryBuilder::new("SELECT bible_id, language, version FROM bible");
    
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

    let rows: Vec<bible::Bible> = query_builder.build_query_as::<bible::Bible>()
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .unwrap()
        })?;

    let result = rows
        .into_iter()
        .map(|bible| format!("{}: {} {}", bible.bible_id, bible.language, bible.version.unwrap()))
        .collect::<Vec<String>>()
        .join(", ");

    Ok(result)
}

pub async fn get_verse_by_search(
    State(app_state): State<AppState>,
    Path(bible_id): Path<i32>,
    Query(params): Query<BibleSearchQueryParams> 
) -> Result<Json<GetVersesRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let mut query_builder = QueryBuilder::new("SELECT * FROM verses WHERE bible_id = ");
    query_builder.push_bind(bible_id);

    if params.book.is_some() || params.chapter.is_some() || params.verse.is_some() || params.text.is_some() {
        query_builder.push(" AND ");
    }

    let mut has_conditions = false;

    if let Some(book) = params.book {
        query_builder.push("book = ").push_bind(book);
        has_conditions = true;
    }

    if let Some(chapter) = params.chapter {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("chapter = ").push_bind(chapter);
        has_conditions = true;
    }

    if let Some(verse) = params.verse {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("verse = ").push_bind(verse);
        has_conditions = true;
    }

    if let Some(text) = &params.text {
        if has_conditions {
            query_builder.push(" AND ");
        }
        query_builder.push("text ILIKE ").push_bind(format!("%{}%", text));
    }

    let rows: Vec<bible::Verse> = query_builder.build_query_as::<bible::Verse>()
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .unwrap()
        })?;

    let verses = rows
        .into_iter()
        .map(|verse| -> Verse {
            Verse {
                bible_id: verse.bible_id,
                book: verse.book,
                chapter: verse.chapter,
                verse: verse.verse,
                text: verse.text
            }
        })
        .collect::<Vec<Verse>>();

    Ok(Json(GetVersesRes { verses }))
}