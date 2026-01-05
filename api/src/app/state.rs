use crate::{
    config,
    db::{self},
    repo::{
        audio_bible::PgAudioBibleRepo,
        bible::PgBibleRepo,
        blob::AwsS3Repo,
    },
    service::{
        audio_bibles::{AudioBibleService, DefaultAudioBibleService},
        bibles::{BibleService, DefaultBibleService},
    },
};
use config::settings::Settings;
use std::{error::Error, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub app_settings: Arc<Settings>,
    pub redis_client: Arc<redis::Client>,
    pub audio_bible_service: Arc<dyn AudioBibleService>,
    pub bible_service: Arc<dyn BibleService>,
}

impl AppState {
    pub async fn get_app_state(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let db_client = Arc::new(db::postgres::get_client(&settings.database_settings).await?);
        let s3_client = Arc::new(db::s3::get_client(&settings.aws_settings).await?);
        let redis_client = Arc::new(db::redis::get_client(&settings.redis_settings).await?);
        let app_settings = Arc::new(settings.clone());

        let bible_repo = Arc::new(PgBibleRepo::new(db_client.clone()));
        let audio_bible_repo = Arc::new(PgAudioBibleRepo::new(db_client.clone()));
        let blob_store = Arc::new(AwsS3Repo::new(s3_client.clone()));

        let audio_bible_service = Arc::new(
            DefaultAudioBibleService::new(
                audio_bible_repo.clone(),
                blob_store.clone(),
                app_settings.clone(),
            )
        );
        let bible_service= Arc::new(DefaultBibleService::new(bible_repo.clone()));

        let app_state = AppState {
            app_settings,
            redis_client,
            audio_bible_service,
            bible_service,
        };
        Ok(app_state)
    }
}
