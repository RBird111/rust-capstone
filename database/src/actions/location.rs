use super::DataResult;
use crate::models::location::{Location, LocationForm};
use crate::schema::locations::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::Value;

pub fn get_all_locations(conn: &mut PgConnection) -> DataResult<Vec<Location>> {
    Ok(locations.select(Location::as_select()).load(conn)?)
}

pub fn get_location_by_id(conn: &mut PgConnection, location_id: i32) -> DataResult<Location> {
    Ok(locations.find(location_id).first(conn)?)
}

pub fn get_location_by_address(conn: &mut PgConnection, loc_address: &str) -> DataResult<Location> {
    Ok(locations
        .select(Location::as_select())
        .filter(address.eq(loc_address))
        .first(conn)?)
}

pub fn create_new_location(
    conn: &mut PgConnection,
    location: LocationForm,
) -> DataResult<Location> {
    Ok(diesel::insert_into(locations)
        .values(location)
        .get_result(conn)?)
}

pub fn update_location(conn: &mut PgConnection, location: Location) -> DataResult<Location> {
    Ok(diesel::update(locations.filter(id.eq(location.id)))
        .set(location)
        .get_result(conn)?)
}

pub fn delete_location(conn: &mut PgConnection, location_id: i32) -> Value {
    let success = r#"{"message": "Location successfully deleted"}"#;
    let error = r#"{"message": "Unable to locate location"}"#;

    match diesel::delete(locations.find(location_id)).execute(conn) {
        Ok(0) => serde_json::from_str(error),
        Ok(_) => serde_json::from_str(success),
        Err(_) => serde_json::from_str(error),
    }
    .expect("error serializing message")
}
