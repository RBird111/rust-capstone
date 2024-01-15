use super::location::Location;
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

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = businesses)]
pub struct BusinessForm {
    pub name: String,
    pub description: String,
    pub category: String,
    pub owner_id: Option<i32>,
}
