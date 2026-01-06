use crate::config::settings::Config;
use crate::service::audio_bibles::interface::AudioBibleService;
use crate::{
    models::http::response::audio_bibles::audio_bibles::AudioBible,
    repo::{
        audio_bible::AudioBibleRepo,
        blob::{BlobObject, BlobStore},
    },
};
use std::{error::Error, sync::Arc};

pub struct DefaultAudioBibleService {
    audio_bible_repo: Arc<dyn AudioBibleRepo>,
    blob_store: Arc<dyn BlobStore>,
    app_config: Arc<Config>,
}

impl DefaultAudioBibleService {
    pub fn new(
        audio_bible_repo: Arc<dyn AudioBibleRepo>,
        blob_store: Arc<dyn BlobStore>,
        app_config: Arc<Config>,
    ) -> Self {
        Self {
            audio_bible_repo,
            blob_store,
            app_config,
        }
    }
}

#[async_trait::async_trait]
impl AudioBibleService for DefaultAudioBibleService {
    async fn get_audio_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<AudioBible>, Box<dyn Error>> {
        self.audio_bible_repo
            .get_audio_bibles(language, version)
            .await
    }

    async fn get_audio_bible_books(&self, audio_bible_id: i32) -> Result<i64, Box<dyn Error>> {
        self.audio_bible_repo
            .get_audio_bible_books(audio_bible_id)
            .await
    }

    async fn get_audio_bible_chapters(
        &self,
        audio_bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        self.audio_bible_repo
            .get_audio_bible_chapters(audio_bible_id, book_num)
            .await
    }

    async fn get_audio_chapter(
        &self,
        audio_bible_id: i32,
        book_num: i32,
        chapter_num: i32,
    ) -> Result<BlobObject, Box<dyn Error>> {
        let file_key = self
            .audio_bible_repo
            .get_audio_chapter_file_key(audio_bible_id, book_num, chapter_num)
            .await?;

        self.blob_store
            .get_object(&self.app_config.aws_config.audio_bibles_bucket, &file_key)
            .await
    }
}
