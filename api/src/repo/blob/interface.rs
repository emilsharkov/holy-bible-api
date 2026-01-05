use aws_sdk_s3::primitives::ByteStream;
use std::error::Error;

pub struct BlobObject {
    pub content: ByteStream,
}

impl BlobObject {
    pub fn new(content: ByteStream) -> Self {
        Self { content }
    }
}

#[async_trait::async_trait]
pub trait BlobStore: Send + Sync {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<BlobObject, Box<dyn Error>>;
}

// Note: S3Repo is deprecated, use BlobStore instead
