pub mod aws;
pub mod mock;
pub mod interface;

pub use interface::{BlobObject, BlobStore};
pub use aws::AwsS3Repo;

