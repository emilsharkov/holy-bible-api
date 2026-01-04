use crate::{
    config,
    db::{self, s3::S3Client},
    service::audio_bibles::AudioBibleService,
    service::bibles::BibleService,
};
use config::settings::Settings;
use std::{error::Error, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Arc<redis::Client>,
    pub audio_bible_service: Arc<AudioBibleService>,
    pub bible_service: Arc<BibleService>,
}

impl AppState {
    pub async fn get_app_state(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let db_client = Arc::new(db::postgres::get_client(&settings.database_settings).await?);
        let s3_client = Arc::new(db::s3::get_client(&settings.aws_settings).await?);
        let redis_client = Arc::new(db::redis::get_client(&settings.redis_settings).await?);

        let audio_bible_service =
            Arc::new(AudioBibleService::new(db_client.clone(), s3_client.clone()));
        let bible_service = Arc::new(BibleService::new(db_client.clone()));

        let app_state = AppState {
            redis_client,
            audio_bible_service,
            bible_service,
        };
        Ok(app_state)
    }
}
