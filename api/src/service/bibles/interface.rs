use crate::models::http::response::bibles::{bibles::Bible, verses::BibleVerse};
use std::error::Error;

#[async_trait::async_trait]
pub trait BibleService: Send + Sync {
    async fn get_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<Bible>, Box<dyn Error>>;

    async fn get_bible_books(&self, bible_id: i32) -> Result<i64, Box<dyn Error>>;

    async fn get_bible_chapters(&self, bible_id: i32, book_num: i32) -> Result<i64, Box<dyn Error>>;

    async fn get_bible_verses(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        start: i32,
        end: i32,
    ) -> Result<Vec<BibleVerse>, Box<dyn Error>>;

    async fn get_bible_verse_by_number(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verse_num: i32,
    ) -> Result<BibleVerse, Box<dyn Error>>;

    async fn get_random_bible_verse(
        &self,
        bible_id: i32,
        seed: Option<f64>,
    ) -> Result<BibleVerse, Box<dyn Error>>;
}

