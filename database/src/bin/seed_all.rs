use database::models;
use database::schema::businesses;
use database::schema::locations;
use database::schema::users;
use diesel::prelude::*;
use diesel::PgConnection;
use rand::seq::SliceRandom;
use serde_json::{from_str, Value};

fn main() {
    let mut conn = database::establish_connection();

    truncate_tables(&mut conn);
    seed_users(&mut conn);
    seed_locations(&mut conn);
    seed_businesses(&mut conn);
}

fn truncate_tables(conn: &mut PgConnection) {
    println!("Truncating tables...");

    diesel::delete(businesses::table)
        .execute(conn)
        .expect("error truncating businesses table");

    diesel::delete(locations::table)
        .execute(conn)
        .expect("error truncating locations table");

    diesel::delete(users::table)
        .execute(conn)
        .expect("error truncating users table");

    println!("Tables truncated.");
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
    use database::actions::location::get_all_locations;
    use database::schema::businesses::dsl::*;
    use models::business::*;

    println!("\nSeeding businesses table...");

    let mut rng = rand::thread_rng();

    let mut locations =
        get_all_locations(conn).expect("error getting locations -- seed_businesses");
    locations.shuffle(&mut rng);

    let data = include_str!("./seed_data/businesses.json");
    let business_json: Vec<BusinessData> = from_str(data).expect("error parsing json");
    let business_data: Vec<BusinessInsertable> = business_json
        .into_iter()
        .zip(locations)
        .map(|(data, loc)| BusinessInsertable::new(data, loc.id))
        .collect();

    diesel::insert_into(businesses)
        .values(business_data)
        .execute(conn)
        .expect("error inserting businesses");

    println!("Businesses table seeded.");
}
