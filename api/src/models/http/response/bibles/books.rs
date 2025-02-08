use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetBibleBooksRes {
    pub num_books: i64,
}
