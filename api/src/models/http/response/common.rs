use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct BooksCountResponse {
    pub num_books: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChaptersCountResponse {
    pub num_chapters: i64,
}
