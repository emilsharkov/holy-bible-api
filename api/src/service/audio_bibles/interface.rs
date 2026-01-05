use crate::{
    models::http::response::audio_bibles::audio_bibles::AudioBible,
    repo::blob::BlobObject,
};
use std::error::Error;

#[async_trait::async_trait]
pub trait AudioBibleService: Send + Sync {
    async fn get_audio_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<AudioBible>, Box<dyn Error>>;

    async fn get_audio_bible_books(&self, audio_bible_id: i32) -> Result<i64, Box<dyn Error>>;

    async fn get_audio_bible_chapters(
        &self,
        audio_bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>>;

    async fn get_audio_chapter(
        &self,
        audio_bible_id: i32,
        book_num: i32,
        chapter_num: i32,
    ) -> Result<BlobObject, Box<dyn Error>>;
}

