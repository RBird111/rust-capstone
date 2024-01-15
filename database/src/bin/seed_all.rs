use std::fs;

use database::models;
use diesel::prelude::*;
use diesel::PgConnection;
use rand::seq::IteratorRandom;
use serde_json::{from_str, Value};

fn main() {
    let mut conn = database::establish_connection();

    reset_tables(&mut conn);
    seed_users(&mut conn);
    seed_locations(&mut conn);
    seed_businesses(&mut conn);
}

fn reset_tables(conn: &mut PgConnection) {
    println!("Resetting tables...");

    let migrations = "./database/migrations/";
    let directory = fs::read_dir(migrations).unwrap();

    // Collect folder names as Strings
    let mut folders: Vec<String> = directory
        .into_iter()
        .filter_map(std::io::Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| !name.starts_with("00000000000")) // Except the init folder
        .map(|e| format!("{migrations}{e}"))
        .collect();

    folders.sort_unstable();

    // Down
    folders
        .iter()
        .rev()
        .map(|s| format!("{s}/down.sql"))
        .inspect(|s| println!("{s}"))
        .filter_map(|s| fs::read_to_string(s).ok())
        .for_each(|query| {
            diesel::sql_query(query).execute(conn).unwrap();
        });

    // Up
    folders
        .iter()
        .map(|s| format!("{s}/up.sql"))
        .inspect(|s| println!("{s}"))
        .filter_map(|s| fs::read_to_string(s).ok())
        .for_each(|query| {
            diesel::sql_query(query).execute(conn).unwrap();
        });

    println!("Tables reset.");
}

fn seed_users(conn: &mut PgConnection) {
    use database::schema::users::dsl::*;
    use models::user::*;

    println!("\nSeeding users table...");

    let data = include_str!("./seed_data/users.json");
    let user_json: Vec<Value> = from_str(data).expect("error parsing json");
    let user_data: Vec<UserForm> = user_json.into_iter().map(UserForm::from_json).collect();

    diesel::insert_into(users)
        .values(user_data)
        .execute(conn)
        .expect("error inserting users");

    println!("Users table seeded.");
}

fn seed_locations(conn: &mut PgConnection) {
    use database::schema::locations::dsl::*;
    use models::location::*;

    println!("\nSeeding locations table...");

    let data = include_str!("./seed_data/locations.json");
    let location_data: Vec<LocationForm> = from_str(data).expect("error parsing json");

    diesel::insert_into(locations)
        .values(location_data)
        .execute(conn)
        .expect("error inserting users");

    println!("Locations table seeded.");
}

fn seed_businesses(conn: &mut PgConnection) {
    use database::schema::businesses::dsl::*;
    use models::business::*;

    println!("\nSeeding businesses table...");

    let mut rng = rand::thread_rng();
    let locations = (1..51).choose_multiple(&mut rng, 30);
    let users = (1..51).choose_multiple(&mut rng, 30);

    let data = include_str!("./seed_data/businesses.json");
    let business_json: Vec<BusinessData> = from_str(data).expect("error parsing json");
    let business_data: Vec<BusinessInsertable> = business_json
        .into_iter()
        .zip(users)
        .map(|(data, owner)| BusinessData {
            owner_id: Some(owner),
            ..data
        })
        .zip(locations)
        .map(|(data, loc)| BusinessInsertable::new(data, loc))
        .collect();

    diesel::insert_into(businesses)
        .values(business_data)
        .execute(conn)
        .expect("error inserting businesses");

    println!("Businesses table seeded.");
}
