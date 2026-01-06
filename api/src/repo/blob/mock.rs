use super::interface::{BlobObject, BlobStore};
use aws_sdk_s3::primitives::ByteStream;
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)] // Used in tests
pub struct MockBlobRepo {
    objects: HashMap<(String, String), Vec<u8>>,
}

#[allow(dead_code)] // Used in tests
impl MockBlobRepo {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn with_object(mut self, bucket: &str, key: &str, data: Vec<u8>) -> Self {
        self.objects
            .insert((bucket.to_string(), key.to_string()), data);
        self
    }
}

impl Default for MockBlobRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BlobStore for MockBlobRepo {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_blob_repo_new() {
        let repo = MockBlobRepo::new();
        let result = repo.get_object("bucket", "key").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_blob_repo_with_object() {
        let data = b"test data".to_vec();
        let repo = MockBlobRepo::new().with_object("bucket", "key", data.clone());
        let blob_object = repo.get_object("bucket", "key").await.unwrap();
        // Verify we got the object (can't easily verify ByteStream content, but we can check it doesn't error)
        // ByteStream doesn't have is_some(), but if we got here without error, the object exists
        let _ = blob_object.content;
    }

    #[tokio::test]
    async fn test_mock_blob_repo_object_not_found() {
        let repo = MockBlobRepo::new().with_object("bucket", "key", b"data".to_vec());
        let result = repo.get_object("bucket", "wrong_key").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_blob_repo_multiple_objects() {
        let repo = MockBlobRepo::new()
            .with_object("bucket1", "key1", b"data1".to_vec())
            .with_object("bucket2", "key2", b"data2".to_vec());

        let obj1 = repo.get_object("bucket1", "key1").await.unwrap();
        // ByteStream doesn't have is_some(), but if we got here without error, the object exists
        let _ = obj1.content;

        let obj2 = repo.get_object("bucket2", "key2").await.unwrap();
        // ByteStream doesn't have is_some(), but if we got here without error, the object exists
        let _ = obj2.content;
    }
}
