use crate::service::audio_bibles::interface::AudioBibleService;
use crate::{
    models::http::response::audio_bibles::audio_bibles::AudioBible,
    repo::blob::BlobObject,
};
use std::{collections::HashMap, error::Error};

#[allow(dead_code)] // Used in tests
pub struct MockAudioBibleService {
    audio_bibles: Vec<AudioBible>,
    books: HashMap<i32, i64>,
    chapters: HashMap<(i32, i32), i64>,
    file_keys: HashMap<(i32, i32, i32), String>,
}

#[allow(dead_code)] // Used in tests
impl MockAudioBibleService {
    pub fn new() -> Self {
        Self {
            audio_bibles: vec![],
            books: HashMap::new(),
            chapters: HashMap::new(),
            file_keys: HashMap::new(),
        }
    }

    pub fn with_audio_bibles(mut self, audio_bibles: Vec<AudioBible>) -> Self {
        self.audio_bibles = audio_bibles;
        self
    }

    pub fn with_books(mut self, audio_bible_id: i32, num_books: i64) -> Self {
        self.books.insert(audio_bible_id, num_books);
        self
    }

    pub fn with_chapters(mut self, audio_bible_id: i32, book_num: i32, num_chapters: i64) -> Self {
        self.chapters.insert((audio_bible_id, book_num), num_chapters);
        self
    }

    pub fn with_file_key(mut self, audio_bible_id: i32, book_num: i32, chapter_num: i32, file_key: String) -> Self {
        self.file_keys.insert((audio_bible_id, book_num, chapter_num), file_key);
        self
    }
}

impl Default for MockAudioBibleService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AudioBibleService for MockAudioBibleService {
    async fn get_audio_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<AudioBible>, Box<dyn Error>> {
        let mut result: Vec<AudioBible> = self.audio_bibles.iter().map(|ab| AudioBible {
            audio_bible_id: ab.audio_bible_id,
            language: ab.language.clone(),
            version: ab.version.clone(),
        }).collect();

        if let Some(lang) = language {
            result.retain(|ab| ab.language == lang);
        }

        if let Some(ver) = version {
            result.retain(|ab| ab.version.as_ref().map(|s| s.as_str()) == Some(ver.as_str()));
        }

        Ok(result)
    }

    async fn get_audio_bible_books(&self, audio_bible_id: i32) -> Result<i64, Box<dyn Error>> {
        self.books
            .get(&audio_bible_id)
            .copied()
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))
    }

    async fn get_audio_bible_chapters(
        &self,
        audio_bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        self.chapters
            .get(&(audio_bible_id, book_num))
            .copied()
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))
    }

    async fn get_audio_chapter(
        &self,
        _audio_bible_id: i32,
        _book_num: i32,
        _chapter_num: i32,
    ) -> Result<BlobObject, Box<dyn Error>> {
        // Mock implementation - return error indicating this is a mock
        Err(Box::<dyn Error>::from("MockAudioBibleService: BlobObject construction not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_audio_bible_service_new() {
        let service = MockAudioBibleService::new();
        let audio_bibles = service.get_audio_bibles(None, None).await.unwrap();
        assert_eq!(audio_bibles.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_with_audio_bibles() {
        let audio_bibles = vec![
            AudioBible {
                audio_bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            AudioBible {
                audio_bible_id: 2,
                language: "es".to_string(),
                version: Some("RVR".to_string()),
            },
        ];
        let service = MockAudioBibleService::new().with_audio_bibles(audio_bibles);
        let result = service.get_audio_bibles(None, None).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_filter_by_language() {
        let audio_bibles = vec![
            AudioBible {
                audio_bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            AudioBible {
                audio_bible_id: 2,
                language: "es".to_string(),
                version: Some("RVR".to_string()),
            },
        ];
        let service = MockAudioBibleService::new().with_audio_bibles(audio_bibles);
        let result = service.get_audio_bibles(Some("en".to_string()), None).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "en");
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_filter_by_version() {
        let audio_bibles = vec![
            AudioBible {
                audio_bible_id: 1,
                language: "en".to_string(),
                version: Some("KJV".to_string()),
            },
            AudioBible {
                audio_bible_id: 2,
                language: "en".to_string(),
                version: Some("NIV".to_string()),
            },
        ];
        let service = MockAudioBibleService::new().with_audio_bibles(audio_bibles);
        let result = service.get_audio_bibles(None, Some("KJV".to_string())).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, Some("KJV".to_string()));
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_with_books() {
        let service = MockAudioBibleService::new().with_books(1, 66);
        let num_books = service.get_audio_bible_books(1).await.unwrap();
        assert_eq!(num_books, 66);
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_with_chapters() {
        let service = MockAudioBibleService::new().with_chapters(1, 1, 50);
        let num_chapters = service.get_audio_bible_chapters(1, 1).await.unwrap();
        assert_eq!(num_chapters, 50);
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_with_file_key() {
        let service = MockAudioBibleService::new()
            .with_file_key(1, 1, 1, "test/path/file.mp3".to_string());
        // The with_file_key method is tested by its usage, but get_audio_chapter
        // returns an error in the mock implementation
        let result = service.get_audio_chapter(1, 1, 1).await;
        assert!(result.is_err());
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(error_msg.contains("MockAudioBibleService"));
        }
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_books_not_found() {
        let service = MockAudioBibleService::new();
        let result = service.get_audio_bible_books(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_audio_bible_service_chapters_not_found() {
        let service = MockAudioBibleService::new();
        let result = service.get_audio_bible_chapters(999, 1).await;
        assert!(result.is_err());
    }
}

