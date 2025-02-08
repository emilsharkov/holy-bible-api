use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema)]
#[derive(Serialize)]
pub struct Bible {
    pub bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(ToSchema)]
#[derive(Serialize)]
pub struct GetBibleRes {
    pub bibles: Vec<Bible>,
}