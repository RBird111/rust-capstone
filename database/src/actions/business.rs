use diesel::prelude::*;
use serde_json::Value;

use super::DataResult;
use crate::actions::location::{create_new_location, get_location_by_address};
use crate::models::business::*;
use crate::schema::businesses::dsl::*;

pub fn get_all_businesses(
    conn: &mut PgConnection,
    category_filter: Option<String>,
) -> DataResult<Value> {
    let business_array: Vec<Business> = match category_filter {
        None => businesses.select(Business::as_select()).load(conn)?,
        Some(cat) => businesses
            .select(Business::as_select())
            .filter(category.eq(cat))
            .load(conn)?,
    };
    // let business_array: Vec<Business> = businesses.select(Business::as_select()).load(conn)?;
    Ok(BusinessArray::new(business_array).eager_load(conn)?)
}

pub fn create_new_business(conn: &mut PgConnection, data: BusinessForm) -> DataResult<Value> {
    let BusinessForm { business, location } = data;
    let location = match get_location_by_address(conn, &location.address) {
        Err(_) => create_new_location(conn, location)?,
        Ok(l) => l,
    };

    let new_business: Business = diesel::insert_into(businesses)
        .values(BusinessInsertable::new(business, location.id))
        .get_result(conn)?;

    Ok(new_business.eager_load(conn)?)
}

pub fn get_business_by_id(conn: &mut PgConnection, business_id: i32) -> DataResult<Value> {
    let business: Business = businesses.find(business_id).first(conn)?;
    Ok(business.eager_load(conn)?)
}

pub fn update_business(conn: &mut PgConnection, business: Business) -> DataResult<Value> {
    let updated_business: Business = diesel::update(businesses.filter(id.eq(business.id)))
        .set(business)
        .get_result(conn)?;
    Ok(updated_business.eager_load(conn)?)
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
