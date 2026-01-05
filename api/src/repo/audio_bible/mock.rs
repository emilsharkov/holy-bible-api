use super::trait_def::AudioBibleRepo;
use crate::models::http::response::audio_bibles::audio_bibles::AudioBible;
use std::{collections::HashMap, error::Error};

/// Mock implementation of AudioBibleRepo for testing
pub struct MockAudioBibleRepo {
    audio_bibles: Vec<AudioBible>,
    books: HashMap<i32, i64>,
    chapters: HashMap<(i32, i32), i64>,
}

impl MockAudioBibleRepo {
    pub fn new() -> Self {
        Self {
            audio_bibles: vec![],
            books: HashMap::new(),
            chapters: HashMap::new(),
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
}

impl Default for MockAudioBibleRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AudioBibleRepo for MockAudioBibleRepo {
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

    async fn get_audio_chapter_file_key(
        &self,
        audio_bible_id: i32,
        book_num: i32,
        chapter_num: i32,
    ) -> Result<String, Box<dyn Error>> {
        // Find the audio bible to get its language
        let audio_bible = self
            .audio_bibles
            .iter()
            .find(|ab| ab.audio_bible_id == audio_bible_id)
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?;

        let file_key = format!(
            "languages/{}/{}/{}.mp3",
            audio_bible.language, book_num, chapter_num
        );

        Ok(file_key)
    }
}

