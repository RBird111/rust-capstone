use super::DataResult;
use crate::actions::location::{create_new_location, get_location_by_address};
use crate::models::business::*;
use crate::schema::businesses::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::Value;

pub fn get_all_businesses(conn: &mut PgConnection) -> DataResult<Vec<Business>> {
    Ok(businesses.select(Business::as_select()).load(conn)?)
}

pub fn create_new_business(conn: &mut PgConnection, data: BusinessForm) -> DataResult<Business> {
    let BusinessForm { business, location } = data;
    let location = match get_location_by_address(conn, &location.address) {
        Err(_) => create_new_location(conn, location)?,
        Ok(l) => l,
    };
    Ok(diesel::insert_into(businesses)
        .values(BusinessInsertable::new(business, location.id))
        .get_result(conn)?)
}

pub fn get_business_by_id(conn: &mut PgConnection, business_id: i32) -> DataResult<Business> {
    Ok(businesses.find(business_id).first(conn)?)
}

pub fn update_business(conn: &mut PgConnection, business: Business) -> DataResult<Business> {
    Ok(diesel::update(businesses).set(business).get_result(conn)?)
}

pub fn delete_business(conn: &mut PgConnection, business_id: i32) -> Value {
    let success = r#"{"message": "Business successfully deleted"}"#;
    let error = r#"{"message": "Unable to locate business"}"#;

    match diesel::delete(businesses.find(business_id)).execute(conn) {
        Ok(0) => serde_json::from_str(error),
        Ok(_) => serde_json::from_str(success),
        Err(_) => serde_json::from_str(error),
    }
    .expect("error serializing message")
}
