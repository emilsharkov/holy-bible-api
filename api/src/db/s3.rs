use crate::config::aws::AwsConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use std::error::Error;

pub struct S3Client {
    pub client: aws_sdk_s3::Client,
}

pub async fn get_client(config: &AwsConfig) -> Result<S3Client, Box<dyn Error>> {
    let region = Region::new(config.audio_bibles_bucket_aws_region.clone());
    let credentials =
        Credentials::from_keys(&config.access_key_id, &config.secret_access_key, None);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .credentials_provider(credentials)
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&config);
    Ok(S3Client {
        client,
    })
}
