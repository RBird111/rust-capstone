use crate::models::review::Review;
use crate::models::user::User;
use crate::schema::{businesses, images, reviews};
use crate::{models::business::Business, schema::users};

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

impl Image {
    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "image": self.get_result(conn)?
        }))
    }

    pub fn get_result(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let user = users::table
            .find(self.user_id)
            .select(User::as_select())
            .first(conn)?;

        let business: Option<Business> = match self.business_id {
            Some(id) => businesses::table
                .find(id)
                .select(Business::as_select())
                .first(conn)
                .optional()?,
            None => None,
        };

        let review: Option<Review> = match self.review_id {
            Some(id) => reviews::table
                .find(id)
                .select(Review::as_select())
                .first(conn)
                .optional()?,
            None => None,
        };

        let full_image: ImageFull = ImageFull {
            image: self.clone(),
            user,
            business,
            review,
        };

        Ok(serde_json::to_value(full_image).unwrap())
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = images)]
pub struct ImageForm {
    pub url: String,
    pub user_id: i32,
    pub business_id: Option<i32>,
    pub review_id: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageFull {
    #[serde(flatten)]
    pub image: Image,
    pub user: User,
    pub business: Option<Business>,
    pub review: Option<Review>,
}

pub struct ImageArray(Vec<Image>);

impl ImageArray {
    pub fn new(array: Vec<Image>) -> Self {
        Self(array)
    }

    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "images": self.0.clone().into_iter().filter_map(|i| i.get_result(conn).ok()).collect::<Vec<Value>>()
        }))
    }
}
