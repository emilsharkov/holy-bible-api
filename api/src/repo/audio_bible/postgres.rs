use super::trait_def::AudioBibleRepo;
use crate::models::{
    http::response::audio_bibles::audio_bibles::AudioBible,
    sql,
};
use sqlx::{PgPool, QueryBuilder};
use std::{error::Error, sync::Arc};

/// Postgres implementation of AudioBibleRepo
pub struct PgAudioBibleRepo {
    db: Arc<PgPool>,
}

impl PgAudioBibleRepo {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl AudioBibleRepo for PgAudioBibleRepo {
    async fn get_audio_bibles(
        &self,
        language: Option<String>,
        version: Option<String>,
    ) -> Result<Vec<AudioBible>, Box<dyn Error>> {
        let mut query_builder =
            QueryBuilder::new("SELECT audio_bible_id, language, version FROM audio_bibles");

        let mut has_conditions = false;
        if language.is_some() || version.is_some() {
            query_builder.push(" WHERE ");
        }

        if let Some(language) = &language {
            query_builder.push("language = ").push_bind(language);
            has_conditions = true;
        }

        if let Some(version) = &version {
            if has_conditions {
                query_builder.push(" AND ");
            }
            query_builder.push("version = ").push_bind(version);
        }

        let rows: Vec<sql::audio_bibles::AudioBible> = query_builder
            .build_query_as::<sql::audio_bibles::AudioBible>()
            .fetch_all(&*self.db)
            .await?;

        let audio_bibles = rows
            .into_iter()
            .map(|audio_bible| -> AudioBible {
                AudioBible {
                    audio_bible_id: audio_bible.audio_bible_id,
                    language: audio_bible.language,
                    version: audio_bible.version,
                }
            })
            .collect::<Vec<AudioBible>>();

        Ok(audio_bibles)
    }

    async fn get_audio_bible_books(&self, audio_bible_id: i32) -> Result<i64, Box<dyn Error>> {
        let rows: Vec<sql::audio_bibles::Count> = sqlx::query_as(
            "SELECT COUNT(distinct book) 
            FROM audio_bible_chapters 
            WHERE audio_bible_id = $1",
        )
        .bind(audio_bible_id)
        .fetch_all(&*self.db)
        .await?;

        let num_books = rows
            .into_iter()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
            .count;

        Ok(num_books)
    }

    async fn get_audio_bible_chapters(
        &self,
        audio_bible_id: i32,
        book_num: i32,
    ) -> Result<i64, Box<dyn Error>> {
        let rows: Vec<sql::audio_bibles::Count> = sqlx::query_as(
            "SELECT COUNT(distinct chapter) 
            FROM audio_bible_chapters 
            WHERE audio_bible_id = $1
            AND book = $2",
        )
        .bind(audio_bible_id)
        .bind(book_num)
        .fetch_all(&*self.db)
        .await?;

        let num_chapters = rows
            .into_iter()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
            .count;

        Ok(num_chapters)
    }

    async fn get_audio_chapter_file_key(
        &self,
        audio_bible_id: i32,
        book_num: i32,
        chapter_num: i32,
    ) -> Result<String, Box<dyn Error>> {
        let rows: Vec<sql::audio_bibles::AudioBible> = sqlx::query_as(
            "SELECT audio_bible_id, language, version FROM audio_bibles WHERE audio_bible_id = $1",
        )
        .bind(audio_bible_id)
        .fetch_all(&*self.db)
        .await?;

        let audio_bible_language = rows
            .into_iter()
            .next()
            .ok_or_else(|| Box::<dyn Error>::from("Audio Bible not found"))?
            .language;

        let file_key = format!(
            "languages/{}/{}/{}.mp3",
            audio_bible_language, book_num, chapter_num
        );

        Ok(file_key)
    }
}

