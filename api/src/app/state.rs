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
use config::settings::Config;
use std::{error::Error, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // Kept for potential future use in middleware or handlers
    pub app_config: Arc<Config>,
    pub redis_client: Arc<redis::Client>,
    pub audio_bible_service: Arc<dyn AudioBibleService>,
    pub bible_service: Arc<dyn BibleService>,
}

impl AppState {
    pub async fn get_app_state(config: &Config) -> Result<Self, Box<dyn Error>> {
        let db_client = Arc::new(db::postgres::get_client(&config.database_config).await?);
        let s3_client = Arc::new(db::s3::get_client(&config.aws_config).await?);
        let redis_client = Arc::new(db::redis::get_client(&config.redis_config).await?);
        let app_config = Arc::new(config.clone());

        let bible_repo = Arc::new(PgBibleRepo::new(db_client.clone()));
        let audio_bible_repo = Arc::new(PgAudioBibleRepo::new(db_client.clone()));
        let blob_store = Arc::new(AwsS3Repo::new(s3_client.clone()));

        let audio_bible_service = Arc::new(
            DefaultAudioBibleService::new(
                audio_bible_repo.clone(),
                blob_store.clone(),
                app_config.clone(),
            )
        );
        let bible_service= Arc::new(DefaultBibleService::new(bible_repo.clone()));

        let app_state = AppState {
            app_config,
            redis_client,
            audio_bible_service,
            bible_service,
        };
        Ok(app_state)
    }
}
