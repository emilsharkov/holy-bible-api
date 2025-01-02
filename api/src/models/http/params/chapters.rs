use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChaptersPathParams {
    pub bible_id: String,
    pub book_num: i32,
}