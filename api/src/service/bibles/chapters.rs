use crate::models::sql::bible;
use sqlx::PgPool;
use std::error::Error;

pub async fn get_bible_chapters(
    db_client: &PgPool,
    bible_id: i32,
    book_num: i32,
) -> Result<i64, Box<dyn Error>> {
    let rows: Vec<bible::Count> = sqlx::query_as(
        "SELECT count(distinct chapter) FROM verses WHERE bible_id = $1 AND book = $2",
    )
    .bind(bible_id)
    .bind(book_num)
    .fetch_all(db_client)
    .await?;

    let num_chapters = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Bible book not found"))?
        .count;

    Ok(num_chapters)
}
