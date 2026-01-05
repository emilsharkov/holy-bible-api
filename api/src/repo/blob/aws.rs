use super::interface::{BlobObject, BlobStore};
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use std::{error::Error, sync::Arc};
use tracing;

use crate::db::s3::S3Client;

pub struct AwsS3Repo {
    client: Arc<S3Client>,
}

impl AwsS3Repo {
    pub fn new(client: Arc<S3Client>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl BlobStore for AwsS3Repo {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<BlobObject, Box<dyn Error>> {
        let S3Client { client } = &*self.client;

        let object_output = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|err| {
                match &err {
                    SdkError::ServiceError(service_err) => {
                        tracing::error!(
                            "S3 Service Error: {:?}, Message: {:?}",
                            service_err.err().code(),
                            service_err.err().message()
                        );
                    }
                    SdkError::TimeoutError(_) => {
                        tracing::error!("S3 request timed out.");
                    }
                    SdkError::DispatchFailure(dispatch_err) => {
                        tracing::error!("Failed to send request: {:?}", dispatch_err);
                    }
                    SdkError::ResponseError(response_err) => {
                        tracing::error!("Invalid response from S3: {:?}", response_err);
                    }
                    _ => {
                        tracing::error!("Unhandled S3 error: {:?}", err);
                    }
                }
                Box::<dyn Error>::from("Resource does not exist")
            })?;

        Ok(BlobObject::new(object_output.body))
    }
}

