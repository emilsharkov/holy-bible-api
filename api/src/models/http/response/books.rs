use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetBooksRes {
    pub num_books: i32,
}
