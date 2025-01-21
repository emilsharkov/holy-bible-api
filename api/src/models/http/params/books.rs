use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookPathParams {
    pub bible_id: i32,
}