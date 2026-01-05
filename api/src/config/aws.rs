use std::error::Error;

#[derive(Debug, Clone)]
pub struct AwsConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub audio_bibles_bucket: String,
    pub audio_bibles_bucket_aws_region: String,
}

impl AwsConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let access_key_id = std::env::var("S3_AWS_ACCESS_KEY_ID")
            .map_err(|_| "Missing required environment variable: S3_AWS_ACCESS_KEY_ID")?;
        let secret_access_key = std::env::var("S3_AWS_SECRET_ACCESS_KEY")
            .map_err(|_| "Missing required environment variable: S3_AWS_SECRET_ACCESS_KEY")?;
        let audio_bibles_bucket = std::env::var("AUDIO_BIBLES_BUCKET")
            .map_err(|_| "Missing required environment variable: AUDIO_BIBLES_BUCKET")?;
        let audio_bibles_bucket_aws_region = std::env::var("AUDIO_BIBLES_BUCKET_AWS_REGION")
            .map_err(|_| "Missing required environment variable: AUDIO_BIBLES_BUCKET_AWS_REGION")?;

        Ok(AwsConfig {
            access_key_id,
            secret_access_key,
            audio_bibles_bucket,
            audio_bibles_bucket_aws_region,
        })
    }
}

