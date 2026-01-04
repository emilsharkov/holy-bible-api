use crate::{models::http::response::bibles::verses::BibleVerse, service::bibles};
use sqlx::PgPool;
use std::{error::Error, sync::Arc};

pub struct BibleService {
    db: Arc<PgPool>,
}

impl BibleService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn get_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<crate::models::http::response::bibles::bibles::Bible>, Box<dyn Error>> {
        bibles::get_bibles(&self.db, language, version).await
    }

    pub async fn get_bible_books(&self, bible_id: i32) -> Result<i64, Box<dyn Error>> {
        bibles::get_bible_books(&self.db, bible_id).await
    }

    pub async fn get_bible_chapters(
        &self,
        bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        bibles::get_bible_chapters(&self.db, bible_id, book_num).await
    }

    pub async fn get_bible_verses(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        start: i32,
        end: i32,
    ) -> Result<Vec<BibleVerse>, Box<dyn Error>> {
        bibles::get_bible_verses(&self.db, bible_id, book_num, chapter_num, start, end)
            .await
    }

    pub async fn get_bible_verse_by_number(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verse_num: i32,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        bibles::get_bible_verse_by_number(
            &self.db,
            bible_id,
            book_num,
            chapter_num,
            verse_num,
        )
        .await
    }
}
