use database::models;
use database::schema::users;
use diesel::prelude::*;
use diesel::PgConnection;
// use rand::seq::SliceRandom;
// use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

fn main() {
    let mut conn = database::establish_connection();

    truncate_tables(&mut conn);
    seed_users(&mut conn);
}

fn truncate_tables(conn: &mut PgConnection) {
    println!("Truncating tables...");

    diesel::delete(users::table)
        .execute(conn)
        .expect("error truncating users table");

    println!("Tables truncated.");
}

fn seed_users(conn: &mut PgConnection) {
    use database::schema::users::dsl::*;
    use models::user::*;

    println!("Seeding users table...");

    let data = include_str!("./seed_data/users.json");
    let user_json: Vec<Value> = from_str(data).expect("error parsing json");
    let user_data: Vec<UserForm> = user_json.into_iter().map(UserForm::from_json).collect();

    diesel::insert_into(users)
        .values(user_data)
        .execute(conn)
        .expect("error inserting users");

    println!("Users table seeded.");
}
