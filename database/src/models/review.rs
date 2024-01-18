use crate::models::business::Business;
use crate::models::user::User;
use crate::schema::reviews;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::image::Image;

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
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
    pub id: i32,
    pub rating: i32,
    pub body: String,
    pub user_id: i32,
    pub business_id: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = reviews)]
pub struct ReviewForm {
    pub rating: i32,
    pub body: String,
    pub user_id: i32,
    pub business_id: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ReviewFull {
    #[serde(flatten)]
    pub review: Review,
    pub user: User,
    pub business: Business,
    pub images: Vec<Image>,
}
