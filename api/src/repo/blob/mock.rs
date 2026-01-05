use super::interface::{BlobObject, BlobStore};
use aws_sdk_s3::primitives::ByteStream;
use std::collections::HashMap;
use std::error::Error;

pub struct MockS3Repo {
    objects: HashMap<(String, String), Vec<u8>>,
}

impl MockS3Repo {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn with_object(mut self, bucket: &str, key: &str, data: Vec<u8>) -> Self {
        self.objects.insert((bucket.to_string(), key.to_string()), data);
        self
    }
}

impl Default for MockS3Repo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BlobStore for MockS3Repo {
    async fn get_object(&self, bucket: &str, key: &str) -> Result<BlobObject, Box<dyn Error>> {
        let key_tuple = (bucket.to_string(), key.to_string());
        if let Some(data) = self.objects.get(&key_tuple) {
            let body = ByteStream::from(data.clone());
            Ok(BlobObject::new(body))
        } else {
            Err(Box::<dyn Error>::from("Object not found"))
        }
    }
}

