use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use crate::config::settings::AwsSettings;
use std::error::Error;
use tracing::info;

pub struct S3Client {
    pub client: aws_sdk_s3::Client,
    pub audio_bibles_bucket: String,
}

pub async fn get_client(settings: &AwsSettings) -> Result<S3Client, Box<dyn Error>> {
    let region = Region::new(settings.audio_bibles_bucket_aws_region.clone());
    let credentials = Credentials::from_keys(
        &settings.access_key_id, 
        &settings.secret_access_key, 
        None
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .credentials_provider(credentials)
        .load()
        .await;
    
    let client = aws_sdk_s3::Client::new(&config);
    let audio_bibles_bucket = settings.audio_bibles_bucket.to_string();
    info!("Connected to s3");
    Ok( S3Client { client,audio_bibles_bucket })
}