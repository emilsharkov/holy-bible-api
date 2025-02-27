use axum::extract::{Query, State};
use axum::Json;
use sqlx::{QueryBuilder, PgPool};
use crate::app::state::AppState;
use crate::models::http::params::bibles::bible::BibleQueryParams;
use crate::models::http::response::bibles::bibles::{Bible, GetBiblesRes};
use crate::models::sql;

#[utoipa::path(
    get,
    path = "/bibles",
    params(BibleQueryParams),
    responses(
        (status = 200, body = GetBiblesRes)
    )
)]
pub async fn get_bibles(
    State(app_state): State<AppState>,
    Query(params): Query<BibleQueryParams>
) -> Result<Json<GetBiblesRes>, axum::response::Response> {
    let db_client: &PgPool = &app_state.db_client;

    let mut query_builder = QueryBuilder::new("SELECT bible_id, language, version FROM bibles");
    
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

    let rows: Vec<sql::bible::Bible> = query_builder.build_query_as::<sql::bible::Bible>()
        .fetch_all(db_client)
        .await
        .map_err(|err| {
            axum::response::Response::builder()
                .status(500)
                .body(format!("Database query failed: {}", err).into())
                .expect("axum response builder failed")
        })?;

    let bibles = rows
        .into_iter()
        .map(|bible| -> Bible { Bible {
            bible_id: bible.bible_id,
            language: bible.language,
            version: bible.version
        }})
        .collect::<Vec<Bible>>();

    Ok(Json(GetBiblesRes { bibles }))
}