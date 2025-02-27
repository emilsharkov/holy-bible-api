use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetAudioBooksRes {
    pub num_books: i64,
}
