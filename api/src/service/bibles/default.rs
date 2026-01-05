use crate::service::bibles::interface::BibleService;
use crate::{
    models::http::response::bibles::verses::BibleVerse,
    repo::bible::BibleRepo,
};
use std::{error::Error, sync::Arc};

pub struct DefaultBibleService {
    bible_repo: Arc<dyn BibleRepo>,
}

impl DefaultBibleService {
    pub fn new(bible_repo: Arc<dyn BibleRepo>) -> Self {
        Self { bible_repo }
    }
}

#[async_trait::async_trait]
impl BibleService for DefaultBibleService {
    async fn get_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<crate::models::http::response::bibles::bibles::Bible>, Box<dyn Error>> {
        self.bible_repo.get_bibles(language, version).await
    }

    async fn get_bible_books(&self, bible_id: i32) -> Result<i64, Box<dyn Error>> {
        self.bible_repo.get_bible_books(bible_id).await
    }

    async fn get_bible_chapters(&self, bible_id: i32, book_num: i32) -> Result<i64, Box<dyn Error>> {
        self.bible_repo.get_bible_chapters(bible_id, book_num).await
    }

    async fn get_bible_verses(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        start: i32,
        end: i32,
    ) -> Result<Vec<BibleVerse>, Box<dyn Error>> {
        self.bible_repo
            .get_bible_verses(bible_id, book_num, chapter_num, start, end)
            .await
    }

    async fn get_bible_verse_by_number(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verse_num: i32,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        self.bible_repo
            .get_bible_verse_by_number(bible_id, book_num, chapter_num, verse_num)
            .await
    }

    async fn get_random_bible_verse(
        &self,
        bible_id: i32,
        seed: Option<f64>,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        self.bible_repo.get_random_bible_verse(bible_id, seed).await
    }
}

