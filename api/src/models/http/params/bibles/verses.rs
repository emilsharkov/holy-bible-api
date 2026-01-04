use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
#[into_params(parameter_in = Query)]
pub struct VersesQuery {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct VersesPath {
    pub bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct VerseByNumberPath {
    pub bible_id: i32,
    pub book_num: i32,
    pub chapter_num: i32,
    pub verse_num: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct RandomBibleVersePath {
    pub bible_id: i32,
}

#[derive(ToSchema, IntoParams, Deserialize, Serialize)]
pub struct VerseOfTheDayPath {
    pub bible_id: i32,
}