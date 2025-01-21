use serde::Serialize;

#[derive(Serialize)]
pub struct Bible {
    pub bible_id: i32,
    pub language: String,
    pub version: Option<String>, // Nullable column in the database
}

#[derive(Serialize)]
pub struct GetBibleRes {
    pub bibles: Vec<Bible>,
}