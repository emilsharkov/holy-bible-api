use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BibleQueryParams {
    pub language: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize)]
pub struct BibleSearchQueryParams {
    pub book: Option<i32>,
    pub chapter: Option<i32>,
    pub verse: Option<i32>,
    pub text: Option<String>
}
