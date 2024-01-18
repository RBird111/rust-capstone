use crate::models::business::Business;
use crate::models::review::Review;
use crate::models::user::User;
use crate::schema::images;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Associations,
    Queryable,
    AsChangeset,
    Selectable,
    Identifiable,
    Serialize,
    Debug,
    Clone,
    Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Business))]
#[diesel(belongs_to(Review))]
#[diesel(table_name = images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    pub id: i32,
    pub url: String,
    pub user_id: i32,
    pub business_id: Option<i32>,
    pub review_id: Option<i32>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = images)]
pub struct ImageForm {
    pub url: String,
    pub user_id: i32,
    pub business_id: Option<i32>,
    pub review_id: Option<i32>,
}
