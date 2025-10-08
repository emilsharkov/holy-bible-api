use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct Bible {
    pub bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(ToSchema, Serialize)]
pub struct GetBiblesRes {
    pub bibles: Vec<Bible>,
}
