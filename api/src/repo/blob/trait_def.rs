use aws_sdk_s3::primitives::ByteStream;
use std::error::Error;

/// Generic blob object returned from blob store operations
pub struct BlobObject {
    pub content: ByteStream,
}

impl BlobObject {
    pub fn new(content: ByteStream) -> Self {
        Self { content }
    }
}

/// Generic repository trait for blob store operations
/// This trait can be implemented for various blob storage backends
/// such as AWS S3, Cloudflare R2, Azure Blob Storage, etc.
#[async_trait::async_trait]
pub trait BlobStore: Send + Sync {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<BlobObject, Box<dyn Error>>;
}

// Note: S3Repo is deprecated, use BlobStore instead
