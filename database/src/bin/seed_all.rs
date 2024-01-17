use std::fs;

use database::{models, schema};
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
    seed_users_locations(&mut conn);
    seed_reviews(&mut conn);
    seed_images(&mut conn)
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
        // except the init folder
        .filter(|name| !name.starts_with("00000000000"))
        // and the .keep file
        .filter(|name| name != ".keep")
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
    use models::user::*;
    use schema::users::dsl::*;

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
    use models::location::*;
    use schema::locations::dsl::*;

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
    use models::business::*;
    use schema::businesses::dsl::*;

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

fn seed_users_locations(conn: &mut PgConnection) {
    use models::users_locations::*;
    use schema::users_locations::dsl::*;

    println!("\nSeeding users_locations table...");

    let mut rng = rand::thread_rng();
    let user_locations: Vec<_> = (1..51)
        .flat_map(|loc_id| (1..51).map(move |u_id| (u_id, loc_id)))
        .choose_multiple(&mut rng, 50)
        .into_iter()
        .map(|(u, l)| UserLocation {
            user_id: u,
            location_id: l,
        })
        .collect();

    diesel::insert_into(users_locations)
        .values(user_locations)
        .execute(conn)
        .expect("error inserting users_locations");

    println!("Users_locations table seeded.");
}

fn seed_reviews(conn: &mut PgConnection) {
    use models::review::ReviewForm;
    use schema::reviews::dsl::*;

    println!("\nSeeding reviews table...");

    let data = include_str!("./seed_data/reviews.json");
    let review_data: Vec<ReviewForm> = from_str(data).expect("error paring json");

    diesel::insert_into(reviews)
        .values(review_data)
        .execute(conn)
        .expect("error inserting reviews");

    println!("Reviews table seeded.");
}

fn seed_images(conn: &mut PgConnection) {
    use models::image::ImageForm;
    use schema::images::dsl::*;

    println!("\nSeeding images table...");

    let extract_url = |val: Value| {
        val["url"]
            .as_str()
            .expect("Error parsing image data.")
            .to_string()
    };

    let data = include_str!("./seed_data/images.json");
    let image_json: Vec<Value> = from_str(data).expect("error parsing json");
    let image_data: Vec<ImageForm> = image_json
        .into_iter()
        .map(extract_url)
        .zip(0..)
        .map(|(url_, idx)| ImageForm {
            url: url_,
            user_id: (idx % 50) + 1,
            business_id: Some((idx % 30) + 1),
            review_id: Some(idx + 1),
        })
        .collect();

    diesel::insert_into(images)
        .values(image_data)
        .execute(conn)
        .expect("error inserting images");

    println!("Images table seeded.");
}
