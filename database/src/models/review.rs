use super::business::Business;
use super::image::Image;
use super::user::User;

use crate::schema::{businesses, images, reviews, users};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "review": self.get_result(conn)?
        }))
    }

    pub fn get_result(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let business: Business = businesses::table.find(self.business_id).first(conn)?;
        let user: User = users::table.find(self.user_id).first(conn)?;
        let images: Vec<Image> = images::table
            .select(Image::as_select())
            .filter(images::review_id.eq(self.id))
            .load(conn)?;

        let result = serde_json::to_value(ReviewFull {
            review: self.clone(),
            user,
            business,
            images,
        })
        .unwrap();

        Ok(result)
    }

    pub fn into_full(&self, conn: &mut PgConnection) -> QueryResult<ReviewFull> {
        let business: Business = businesses::table.find(self.business_id).first(conn)?;
        let user: User = users::table.find(self.user_id).first(conn)?;
        let images: Vec<Image> = images::table
            .select(Image::as_select())
            .filter(images::review_id.eq(self.id))
            .load(conn)?;

        Ok(ReviewFull {
            review: self.clone(),
            user,
            business,
            images,
        })
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

#[derive(Debug, Clone, Serialize)]
pub struct ReviewFull {
    #[serde(flatten)]
    pub review: Review,
    pub business: Business,
    pub user: User,
    pub images: Vec<Image>,
}

#[derive(Debug, Clone)]
pub struct ReviewArray(Vec<Review>);

impl ReviewArray {
    pub fn new(reviews: Vec<Review>) -> Self {
        Self(reviews)
    }

    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let result: Vec<Value> = self
            .0
            .clone()
            .into_iter()
            .filter_map(|r| r.get_result(conn).ok())
            .collect();

        Ok(serde_json::json!({
            "reviews": result
        }))
    }
}
