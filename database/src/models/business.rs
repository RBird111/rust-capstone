use super::image::Image;
use super::location::{Location, LocationForm};
use super::review::{Review, ReviewFull};
use super::user::User;
use crate::schema::{businesses, locations, users};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl Business {
    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "business": self.get_result(conn)?
        }))
    }

    pub fn get_result(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let owner: Option<User> = match self.owner_id {
            Some(id) => users::table
                .find(id)
                .select(User::as_select())
                .first(conn)
                .optional()?,
            None => None,
        };

        let location: Location = locations::table
            .select(Location::as_select())
            .filter(locations::id.eq(self.location_id))
            .first(conn)?;

        let images: Vec<Image> = Image::belonging_to(self)
            .select(Image::as_select())
            .load(conn)?;

        let reviews: Vec<Review> = Review::belonging_to(self)
            .select(Review::as_select())
            .load(conn)?;

        let (sum, count) = reviews
            .iter()
            .map(|r| r.rating as f64)
            .fold((0., 0.), |(s, c), r| (s + r, c + 1.));
        let avg_rating = sum / count;

        let reviews: Vec<ReviewFull> = reviews
            .into_iter()
            .filter_map(|r| r.into_full(conn).ok())
            .collect();

        let full_business = BusinessFull {
            business: self.clone(),
            owner,
            location,
            avg_rating,
            reviews,
            images,
        };

        Ok(serde_json::to_value(full_business).unwrap())
    }
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
    pub fn new(
        BusinessData {
            name,
            description,
            category,
            owner_id,
        }: BusinessData,
        location_id: i32,
    ) -> Self {
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

#[derive(Serialize, Debug, Clone)]
pub struct BusinessFull {
    #[serde(flatten)]
    pub business: Business,
    pub owner: Option<User>,
    pub location: Location,
    pub avg_rating: f64,
    pub reviews: Vec<ReviewFull>,
    pub images: Vec<Image>,
}

pub struct BusinessArray(Vec<Business>);

impl BusinessArray {
    pub fn new(array: Vec<Business>) -> Self {
        Self(array)
    }

    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "businesses": self.0.clone().into_iter().filter_map(|b| b.get_result(conn).ok()).collect::<Vec<Value>>()
        }))
    }
}
