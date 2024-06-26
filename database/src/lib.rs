pub mod actions;
pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    let _ = dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

pub fn get_pool() -> ConnectionPool {
    let _ = dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("error building connection pool")
}
