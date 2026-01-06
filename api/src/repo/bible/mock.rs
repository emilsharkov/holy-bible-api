use super::interface::BibleRepo;
use crate::models::http::response::bibles::{bibles::Bible, verses::BibleVerse};
use std::{collections::HashMap, error::Error};

#[allow(dead_code)] // Used in tests
pub struct MockBibleRepo {
    bibles: Vec<Bible>,
    verses: HashMap<(i32, i32, i32), Vec<BibleVerse>>,
}

#[allow(dead_code)] // Used in tests
impl MockBibleRepo {
    pub fn new() -> Self {
        Self {
            bibles: vec![],
            verses: HashMap::new(),
        }
    }

    pub fn with_bibles(mut self, bibles: Vec<Bible>) -> Self {
        self.bibles = bibles;
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

impl Default for MockBibleRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BibleRepo for MockBibleRepo {
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
        let mut books = std::collections::HashSet::new();
        for ((bid, book_num, _), _) in &self.verses {
            if *bid == bible_id {
                books.insert(*book_num);
            }
        }
        Ok(books.len() as i64)
    }

    async fn get_bible_chapters(
        &self,
        bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        let mut chapters = std::collections::HashSet::new();
        for ((bid, bnum, chapter_num), _) in &self.verses {
            if *bid == bible_id && *bnum == book_num {
                chapters.insert(*chapter_num);
            }
        }
        Ok(chapters.len() as i64)
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
        // Return the first verse found for the given bible_id
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
    async fn test_mock_bible_repo_new() {
        let repo = MockBibleRepo::new();
        let bibles = repo.get_bibles(None, None).await.unwrap();
        assert_eq!(bibles.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_bible_repo_with_bibles() {
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
        let repo = MockBibleRepo::new().with_bibles(bibles);
        let result = repo.get_bibles(None, None).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_bible_repo_filter_by_language() {
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
        let repo = MockBibleRepo::new().with_bibles(bibles);
        let result = repo.get_bibles(Some("en".to_string()), None).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "en");
    }

    #[tokio::test]
    async fn test_mock_bible_repo_filter_by_version() {
        let bibles = vec![
            Bible {
                bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            Bible {
                bible_id: 2,
                language: "en".to_string(),
                version: Some("NIV".to_string()),
            },
        ];
        let repo = MockBibleRepo::new().with_bibles(bibles);
        let result = repo
            .get_bibles(None, Some("KJV".to_string()))
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, Some("KJV".to_string()));
    }

    #[tokio::test]
    async fn test_mock_bible_repo_with_verses() {
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
        let repo = MockBibleRepo::new().with_verses(1, 1, 1, verses);
        let result = repo.get_bible_verses(1, 1, 1, 1, 2).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_bible_repo_get_books() {
        let verse1 = BibleVerse {
            bible_id: 1,
            book: 1,
            chapter: 1,
            verse: 1,
            text: "Verse 1".to_string(),
        };
        let verse2 = BibleVerse {
            bible_id: 1,
            book: 2,
            chapter: 1,
            verse: 1,
            text: "Verse 2".to_string(),
        };
        let repo = MockBibleRepo::new()
            .with_verses(1, 1, 1, vec![verse1])
            .with_verses(1, 2, 1, vec![verse2]);
        let num_books = repo.get_bible_books(1).await.unwrap();
        assert_eq!(num_books, 2);
    }

    #[tokio::test]
    async fn test_mock_bible_repo_get_chapters() {
        let verse1 = BibleVerse {
            bible_id: 1,
            book: 1,
            chapter: 1,
            verse: 1,
            text: "Verse 1".to_string(),
        };
        let verse2 = BibleVerse {
            bible_id: 1,
            book: 1,
            chapter: 2,
            verse: 1,
            text: "Verse 2".to_string(),
        };
        let repo = MockBibleRepo::new()
            .with_verses(1, 1, 1, vec![verse1])
            .with_verses(1, 1, 2, vec![verse2]);
        let num_chapters = repo.get_bible_chapters(1, 1).await.unwrap();
        assert_eq!(num_chapters, 2);
    }

    #[tokio::test]
    async fn test_mock_bible_repo_get_verse_by_number() {
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
        let repo = MockBibleRepo::new().with_verses(1, 1, 1, verses);
        let verse = repo.get_bible_verse_by_number(1, 1, 1, 1).await.unwrap();
        assert_eq!(verse.verse, 1);
        assert_eq!(verse.text, "In the beginning");
    }

    #[tokio::test]
    async fn test_mock_bible_repo_get_random_verse() {
        let verses = vec![BibleVerse {
            bible_id: 1,
            book: 1,
            chapter: 1,
            verse: 1,
            text: "Random verse".to_string(),
        }];
        let repo = MockBibleRepo::new().with_verses(1, 1, 1, verses);
        let verse = repo.get_random_bible_verse(1, None).await.unwrap();
        assert_eq!(verse.bible_id, 1);
    }
}
