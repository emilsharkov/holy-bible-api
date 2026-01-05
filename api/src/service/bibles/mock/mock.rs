use crate::service::bibles::trait_def::BibleService;
use crate::models::http::response::bibles::{bibles::Bible, verses::BibleVerse};
use std::{collections::HashMap, error::Error};

/// Mock implementation of BibleService for testing
pub struct MockBibleService {
    bibles: Vec<Bible>,
    books: HashMap<i32, i64>,
    chapters: HashMap<(i32, i32), i64>,
    verses: HashMap<(i32, i32, i32), Vec<BibleVerse>>,
}

impl MockBibleService {
    pub fn new() -> Self {
        Self {
            bibles: vec![],
            books: HashMap::new(),
            chapters: HashMap::new(),
            verses: HashMap::new(),
        }
    }

    pub fn with_bibles(mut self, bibles: Vec<Bible>) -> Self {
        self.bibles = bibles;
        self
    }

    pub fn with_books(mut self, bible_id: i32, num_books: i64) -> Self {
        self.books.insert(bible_id, num_books);
        self
    }

    pub fn with_chapters(mut self, bible_id: i32, book_num: i32, num_chapters: i64) -> Self {
        self.chapters.insert((bible_id, book_num), num_chapters);
        self
    }

    pub fn with_verses(mut self, bible_id: i32, book_num: i32, chapter_num: i32, verses: Vec<BibleVerse>) -> Self {
        self.verses.insert((bible_id, book_num, chapter_num), verses);
        self
    }
}

impl Default for MockBibleService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BibleService for MockBibleService {
    async fn get_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<Bible>, Box<dyn Error>> {
        let mut result: Vec<Bible> = self.bibles.iter().map(|b| Bible {
            bible_id: b.bible_id,
            language: b.language.clone(),
            version: b.version.clone(),
        }).collect();

        if let Some(lang) = language {
            result.retain(|b| b.language == lang);
        }

        if let Some(ver) = version {
            result.retain(|b| b.version.as_ref().map(|s| s.as_str()) == Some(ver.as_str()));
        }

        Ok(result)
    }

    async fn get_bible_books(&self, bible_id: i32) -> Result<i64, Box<dyn Error>> {
        self.books
            .get(&bible_id)
            .copied()
            .ok_or_else(|| Box::<dyn Error>::from("Bible not found"))
    }

    async fn get_bible_chapters(&self, bible_id: i32, book_num: i32) -> Result<i64, Box<dyn Error>> {
        self.chapters
            .get(&(bible_id, book_num))
            .copied()
            .ok_or_else(|| Box::<dyn Error>::from("Bible book not found"))
    }

    async fn get_bible_verses(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        start: i32,
        end: i32,
    ) -> Result<Vec<BibleVerse>, Box<dyn Error>> {
        let key = (bible_id, book_num, chapter_num);
        if let Some(verses) = self.verses.get(&key) {
            let filtered: Vec<BibleVerse> = verses
                .iter()
                .filter(|v| v.verse >= start && v.verse <= end)
                .map(|v| BibleVerse {
                    bible_id: v.bible_id,
                    book: v.book,
                    chapter: v.chapter,
                    verse: v.verse,
                    text: v.text.clone(),
                })
                .collect();
            Ok(filtered)
        } else {
            Ok(vec![])
        }
    }

    async fn get_bible_verse_by_number(
        &self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verse_num: i32,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        let key = (bible_id, book_num, chapter_num);
        if let Some(verses) = self.verses.get(&key) {
            verses
                .iter()
                .find(|v| v.verse == verse_num)
                .map(|v| BibleVerse {
                    bible_id: v.bible_id,
                    book: v.book,
                    chapter: v.chapter,
                    verse: v.verse,
                    text: v.text.clone(),
                })
                .ok_or_else(|| Box::<dyn Error>::from("Verse not found"))
        } else {
            Err(Box::<dyn Error>::from("Verse not found"))
        }
    }

    async fn get_random_bible_verse(
        &self,
        bible_id: i32,
        _seed: Option<f64>,
    ) -> Result<BibleVerse, Box<dyn Error>> {
        for ((bid, _, _), verses) in &self.verses {
            if *bid == bible_id && !verses.is_empty() {
                let v = &verses[0];
                return Ok(BibleVerse {
                    bible_id: v.bible_id,
                    book: v.book,
                    chapter: v.chapter,
                    verse: v.verse,
                    text: v.text.clone(),
                });
            }
        }
        Err(Box::<dyn Error>::from("Verse not found"))
    }
}

