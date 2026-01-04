use std::error::Error;

use crate::models::sql::audio_bibles;
use sqlx::PgPool;

pub async fn get_audio_bible_books(
    db_client: &PgPool,
    audio_bible_id: i32,
) -> Result<i64, Box<dyn Error>> {
    let rows: Vec<audio_bibles::Count> = sqlx::query_as(
        "SELECT COUNT(distinct book) 
        FROM audio_bible_chapters 
        WHERE audio_bible_id = $1",
    )
    .bind(audio_bible_id)
    .fetch_all(db_client)
    .await?;

    let num_books = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
        .count;

    Ok(num_books)
}
