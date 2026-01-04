use crate::models::http::response::bibles::verses::BibleVerse;
use crate::models::sql::bible;
use sqlx::PgPool;
use std::error::Error;

pub async fn get_bible_verses(
    db_client: &PgPool,
    bible_id: i32,
    book_num: i32,
    chapter_num: i32,
    start: i32,
    end: i32,
) -> Result<Vec<BibleVerse>, Box<dyn Error>> {
    let rows: Vec<bible::Verse> = sqlx::query_as(
        "SELECT bible_id, book, chapter, verse, text 
        FROM verses 
        WHERE bible_id = $1 
        AND book = $2 
        AND chapter = $3
        AND verse >= $4
        AND verse <= $5
        ORDER BY verse ASC",
    )
    .bind(bible_id)
    .bind(book_num)
    .bind(chapter_num)
    .bind(start)
    .bind(end)
    .fetch_all(db_client)
    .await?;

    let verses = rows
        .into_iter()
        .map(|verse| -> BibleVerse {
            BibleVerse {
                bible_id: verse.bible_id,
                book: verse.book,
                chapter: verse.chapter,
                verse: verse.verse,
                text: verse.text,
            }
        })
        .collect::<Vec<BibleVerse>>();

    Ok(verses)
}

pub async fn get_bible_verse_by_number(
    db_client: &PgPool,
    bible_id: i32,
    book_num: i32,
    chapter_num: i32,
    verse_num: i32,
) -> Result<BibleVerse, Box<dyn Error>> {
    let rows: Vec<bible::Verse> = sqlx::query_as(
        "SELECT bible_id, book, chapter, verse, text 
            FROM verses  
            WHERE bible_id = $1 
            AND book = $2 
            AND chapter = $3 
            AND verse = $4",
    )
    .bind(bible_id)
    .bind(book_num)
    .bind(chapter_num)
    .bind(verse_num)
    .fetch_all(db_client)
    .await?;

    let verse = rows
        .into_iter()
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Verse not found"))?;

    Ok(BibleVerse {
        bible_id: verse.bible_id,
        book: verse.book,
        chapter: verse.chapter,
        verse: verse.verse,
        text: verse.text,
    })
}
