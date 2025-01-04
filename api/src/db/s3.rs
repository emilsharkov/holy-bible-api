use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use crate::config::settings::AwsSettings;
use std::error::Error;

pub async fn get_client(settings: &AwsSettings) -> Result<Client, Box<dyn Error>> {
    let credentials = Credentials::from_keys(
        &settings.access_key_id, 
        &settings.secret_access_key, 
        None
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .load()
        .await;
    
    let client = Client::new(&config);
    Ok(client)
}