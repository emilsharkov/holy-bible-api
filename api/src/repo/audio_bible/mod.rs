pub mod mock;
pub mod postgres;
pub mod trait_def;

pub use trait_def::AudioBibleRepo;
pub use mock::MockAudioBibleRepo;
pub use postgres::PgAudioBibleRepo;

