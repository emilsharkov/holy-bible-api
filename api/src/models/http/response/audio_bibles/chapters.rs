use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetChaptersRes {
    pub num_chapters: i64,
}