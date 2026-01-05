pub mod mock;
pub mod postgres;
pub mod trait_def;

pub use trait_def::BibleRepo;
pub use mock::MockBibleRepo;
pub use postgres::PgBibleRepo;

