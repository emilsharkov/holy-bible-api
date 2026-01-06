use crate::models::http::response::bibles::{bibles::Bible, verses::BibleVerse};
use crate::service::bibles::interface::BibleService;
use std::{collections::HashMap, error::Error};

#[allow(dead_code)] // Used in tests
pub struct MockBibleService {
    bibles: Vec<Bible>,
    books: HashMap<i32, i64>,
    chapters: HashMap<(i32, i32), i64>,
    verses: HashMap<(i32, i32, i32), Vec<BibleVerse>>,
}

#[allow(dead_code)] // Used in tests
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

    pub fn with_verses(
        mut self,
        bible_id: i32,
        book_num: i32,
        chapter_num: i32,
        verses: Vec<BibleVerse>,
    ) -> Self {
        self.verses
            .insert((bible_id, book_num, chapter_num), verses);
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
        let mut result: Vec<Bible> = self
            .bibles
            .iter()
            .map(|b| Bible {
                bible_id: b.bible_id,
                language: b.language.clone(),
                version: b.version.clone(),
            })
            .collect();

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

    async fn get_bible_chapters(
        &self,
        bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_bible_service_new() {
        let service = MockBibleService::new();
        let bibles = service.get_bibles(None, None).await.unwrap();
        assert_eq!(bibles.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_bible_service_with_bibles() {
        let bibles = vec![
            Bible {
                bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            Bible {
                bible_id: 2,
                language: "es".to_string(),
                version: Some("RVR".to_string()),
            },
        ];
        let service = MockBibleService::new().with_bibles(bibles);
        let result = service.get_bibles(None, None).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_bible_service_filter_by_language() {
        let bibles = vec![
            Bible {
                bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            Bible {
                bible_id: 2,
                language: "es".to_string(),
                version: Some("RVR".to_string()),
            },
        ];
        let service = MockBibleService::new().with_bibles(bibles);
        let result = service
            .get_bibles(Some("en".to_string()), None)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "en");
    }

    #[tokio::test]
    async fn test_mock_bible_service_with_books() {
        let service = MockBibleService::new().with_books(1, 66);
        let num_books = service.get_bible_books(1).await.unwrap();
        assert_eq!(num_books, 66);
    }

    #[tokio::test]
    async fn test_mock_bible_service_with_chapters() {
        let service = MockBibleService::new().with_chapters(1, 1, 50);
        let num_chapters = service.get_bible_chapters(1, 1).await.unwrap();
        assert_eq!(num_chapters, 50);
    }

    #[tokio::test]
    async fn test_mock_bible_service_with_verses() {
        let verses = vec![
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 1,
                text: "In the beginning".to_string(),
            },
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 2,
                text: "God created".to_string(),
            },
        ];
        let service = MockBibleService::new().with_verses(1, 1, 1, verses);
        let result = service.get_bible_verses(1, 1, 1, 1, 2).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_bible_service_get_verse_by_number() {
        let verses = vec![
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 1,
                text: "In the beginning".to_string(),
            },
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 2,
                text: "God created".to_string(),
            },
        ];
        let service = MockBibleService::new().with_verses(1, 1, 1, verses);
        let verse = service.get_bible_verse_by_number(1, 1, 1, 1).await.unwrap();
        assert_eq!(verse.verse, 1);
        assert_eq!(verse.text, "In the beginning");
    }

    #[tokio::test]
    async fn test_mock_bible_service_get_random_verse() {
        let verses = vec![BibleVerse {
            bible_id: 1,
            book: 1,
            chapter: 1,
            verse: 1,
            text: "Random verse".to_string(),
        }];
        let service = MockBibleService::new().with_verses(1, 1, 1, verses);
        let verse = service.get_random_bible_verse(1, None).await.unwrap();
        assert_eq!(verse.bible_id, 1);
    }

    #[tokio::test]
    async fn test_mock_bible_service_verse_range_filtering() {
        let verses = vec![
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 1,
                text: "Verse 1".to_string(),
            },
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 2,
                text: "Verse 2".to_string(),
            },
            BibleVerse {
                bible_id: 1,
                book: 1,
                chapter: 1,
                verse: 3,
                text: "Verse 3".to_string(),
            },
        ];
        let service = MockBibleService::new().with_verses(1, 1, 1, verses);
        let result = service.get_bible_verses(1, 1, 1, 2, 3).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].verse, 2);
        assert_eq!(result[1].verse, 3);
    }
}
