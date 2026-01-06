pub mod interface;
pub mod mock;
pub mod postgres;

pub use interface::BibleRepo;
pub use postgres::PgBibleRepo;
