use crate::{
    db::s3::S3Client, models::http::response::audio_bibles::audio_bibles::AudioBible,
    service::audio_bibles,
};
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use sqlx::PgPool;
use std::{error::Error, sync::Arc};

pub struct AudioBibleService {
    db: Arc<PgPool>,
    s3: Arc<S3Client>,
}

impl AudioBibleService {
    pub fn new(db: Arc<PgPool>, s3: Arc<S3Client>) -> Self {
        Self { db, s3 }
    }

    pub async fn get_audio_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<AudioBible>, Box<dyn Error>> {
        audio_bibles::get_audio_bibles(&self.db, language, version).await
    }

    pub async fn get_audio_bible_books(&self, audio_bible_id: i32) -> Result<i64, Box<dyn Error>> {
        audio_bibles::get_audio_bible_books(&self.db, audio_bible_id).await
    }

    pub async fn get_audio_bible_chapters(
        &self,
        audio_bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        audio_bibles::get_audio_bible_chapters(&self.db, audio_bible_id, book_num).await
    }

    pub async fn get_audio_chapter(
        &self,
        audio_bible_id: i32,
        book_num: i32,
        chapter_num: i32,
    ) -> Result<GetObjectOutput, Box<dyn Error>> {
        audio_bibles::get_audio_chapter_from_s3(
            &self.db,
            &self.s3,
            audio_bible_id,
            book_num,
            chapter_num,
        )
        .await
    }
}
