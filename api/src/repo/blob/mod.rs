pub mod aws;
pub mod mock;
pub mod trait_def;

pub use trait_def::{BlobObject, BlobStore};
pub use aws::AwsS3Repo;
#[cfg(test)]
pub use mock::MockS3Repo;

