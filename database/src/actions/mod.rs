pub mod business;
pub mod location;
pub mod review;
pub mod user;

pub type DataResult<T> = Result<T, diesel::result::Error>;
