use serde::Serialize;
use utoipa::ToSchema;

/// Response containing a count of books
#[derive(Debug, Serialize, ToSchema)]
pub struct BooksCountResponse {
    pub num_books: i64,
}

/// Response containing a count of chapters
#[derive(Debug, Serialize, ToSchema)]
pub struct ChaptersCountResponse {
    pub num_chapters: i64,
}

