use super::location::{Location, LocationForm};
use super::user::User;
use crate::schema::businesses;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Insertable,
    Queryable,
    AsChangeset,
    Selectable,
    Identifiable,
    Associations,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[diesel(table_name = businesses)]
#[diesel(belongs_to(Location))]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Business {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub location_id: i32,
    pub owner_id: Option<i32>,
}

#[derive(Associations, Insertable, Debug, Clone, Serialize)]
#[diesel(table_name = businesses)]
#[diesel(belongs_to(Location))]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BusinessInsertable {
    pub name: String,
    pub description: String,
    pub category: String,
    pub location_id: i32,
    pub owner_id: Option<i32>,
}

impl BusinessInsertable {
    pub fn new(data: BusinessData, location_id: i32) -> Self {
        let BusinessData {
            name,
            description,
            category,
            owner_id,
        } = data;

        Self {
            name,
            description,
            category,
            location_id,
            owner_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessData {
    pub name: String,
    pub description: String,
    pub category: String,
    pub owner_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessForm {
    #[serde(flatten)]
    pub business: BusinessData,
    #[serde(flatten)]
    pub location: LocationForm,
}
