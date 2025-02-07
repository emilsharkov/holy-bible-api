use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChaptersPathParams {
    pub bible_id: i32,
    pub book_num: i32,
}