use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetBibleVersesRes {
    pub verses: Vec<BibleVerse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BibleVerse {
    pub bible_id: i32,
    pub book: i32,
    pub chapter: i32,
    pub verse: i32,
    pub text: String,
}
