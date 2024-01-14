pub mod location;
pub mod user;

pub type DataResult<T> = Result<T, diesel::result::Error>;
