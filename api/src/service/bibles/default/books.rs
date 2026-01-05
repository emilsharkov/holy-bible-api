use crate::models::sql::bible;
use sqlx::PgPool;
use std::error::Error;

pub async fn get_bible_books(db_client: &PgPool, bible_id: i32) -> Result<i64, Box<dyn Error>> {
    let rows: Vec<bible::Count> =
        sqlx::query_as("SELECT count(distinct book) FROM verses WHERE bible_id = $1")
            .bind(bible_id)
            .fetch_all(db_client)
            .await?;

    let num_books = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Bible not found"))?
        .count;

    Ok(num_books)
}
