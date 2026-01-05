pub mod mock;
pub mod postgres;
pub mod interface;

pub use interface::BibleRepo;
pub use postgres::PgBibleRepo;

