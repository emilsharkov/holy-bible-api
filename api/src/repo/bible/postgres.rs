use super::interface::BibleRepo;
use crate::models::{
    http::response::bibles::{bibles::Bible, verses::BibleVerse},
    sql,
};
use sqlx::{PgPool, QueryBuilder};
use std::time;
use std::{error::Error, sync::Arc};

pub struct PgBibleRepo {
    db: Arc<PgPool>,
}

impl PgBibleRepo {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl BibleRepo for PgBibleRepo {
    async fn get_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<Bible>, Box<dyn Error>> {
        let mut query_builder = QueryBuilder::new("SELECT bible_id, language, version FROM bibles");

        let mut has_conditions = false;
        if language.is_some() || version.is_some() {
            query_builder.push(" WHERE ");
        }

        if let Some(lang) = &language {
            query_builder.push("language = ").push_bind(lang);
            has_conditions = true;
        }

        if let Some(ver) = &version {
            if has_conditions {
                query_builder.push(" AND ");
            }
            query_builder.push("version = ").push_bind(ver);
        }

        let rows: Vec<sql::bible::Bible> = query_builder
            .build_query_as::<sql::bible::Bible>()
            .fetch_all(&*self.db)
            .await?;

        let bibles = rows
            .into_iter()
            .map(|bible| -> Bible {
                Bible {
                    bible_id: bible.bible_id,
                    language: bible.language,
                    version: bible.version,
                }
            })
            .collect::<Vec<Bible>>();

        Ok(bibles)
    }

    async fn get_bible_books(&self, bible_id: i32) -> Result<i64, Box<dyn Error>> {
        let rows: Vec<sql::bible::Count> =
            sqlx::query_as("SELECT count(distinct book) FROM verses WHERE bible_id = $1")
                .bind(bible_id)
                .fetch_all(&*self.db)
                .await?;

        let num_books = rows
            .into_iter()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("Bible not found"))?
            .count;

        Ok(num_books)
    }

    async fn get_bible_chapters(
        &self,
        bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        let rows: Vec<sql::bible::Count> = sqlx::query_as(
            "SELECT count(distinct chapter) FROM verses WHERE bible_id = $1 AND book = $2",
        )
        .bind(bible_id)
        .bind(book_num)
        .fetch_all(&*self.db)
        .await?;

        let num_chapters = rows
            .into_iter()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("Bible book not found"))?
            .count;

        Ok(num_chapters)
    }

    async fn get_bible_verses(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        start: i32,
        end: i32,
    ) -> Result<Vec<BibleVerse>, Box<dyn Error>> {
        let rows: Vec<sql::bible::Verse> = sqlx::query_as(
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
        .fetch_all(&*self.db)
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

    async fn get_bible_verse_by_number(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verse_num: i32,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        let rows: Vec<sql::bible::Verse> = sqlx::query_as(
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
        .fetch_all(&*self.db)
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

    async fn get_random_bible_verse(
        &self,
        bible_id: i32,
        seed: Option<f64>,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        let seed = seed.unwrap_or(
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as f64,
        );
        let rows: Vec<sql::bible::Verse> = sqlx::query_as(
            "SELECT bible_id, book, chapter, verse, text 
            FROM verses 
            WHERE bible_id = $1 
            ORDER BY hashtext($2::text || '|' || bible_id::text || '|' || book::text || '|' || chapter::text || '|' || verse::text) LIMIT 1",
        )
        .bind(bible_id)
        .bind(seed)
        .fetch_all(&*self.db)
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
}
