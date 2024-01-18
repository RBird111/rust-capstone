use crate::models::business::Business;
use crate::models::user::User;
use crate::schema::{businesses, images, reviews, users};

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

impl Review {
    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<serde_json::Value> {
        let business: Business = businesses::table.find(self.business_id).first(conn)?;
        let user: User = users::table.find(self.user_id).first(conn)?;
        let images: Vec<Image> = images::table
            .select(Image::as_select())
            .filter(images::review_id.eq(self.id))
            .load(conn)?;

        let result = serde_json::json!({
            "id": self.id,
            "rating": self.rating,
            "body": self.body,
            "user_id": self.user_id,
            "business_id": self.business_id,
            "user": serde_json::to_value(user).unwrap(),
            "business": serde_json::to_value(business).unwrap(),
            "images": serde_json::to_value(images).unwrap()
        });

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = reviews)]
pub struct ReviewForm {
    pub rating: i32,
    pub body: String,
    pub user_id: i32,
    pub business_id: i32,
}
