use super::business::Business;
use super::image::Image;
use super::location::Location;
use super::review::Review;
use super::users_locations::UserLocation;

use crate::schema::{locations, users};

use diesel::prelude::*;
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(
    AsChangeset,
    Queryable,
    Insertable,
    Selectable,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub hashed_password: String,
}

impl User {
    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        Ok(serde_json::json!({
            "user": self.get_result(conn)?
        }))
    }

    pub fn get_result(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let locations = UserLocation::belonging_to(self)
            .inner_join(locations::table)
            .select(Location::as_select())
            .load(conn)?;

        let reviews = Review::belonging_to(self)
            .select(Review::as_select())
            .load(conn)?;

        let images = Image::belonging_to(self)
            .select(Image::as_select())
            .load(conn)?;

        let owned_business = Business::belonging_to(self)
            .select(Business::as_select())
            .load(conn)?;

        let result = serde_json::to_value(UserFull {
            user: self.clone(),
            locations,
            reviews,
            images,
            owned_business,
        })
        .unwrap();

        Ok(result)
    }
}

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UserForm {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

impl UserForm {
    pub fn from_json(user_data: Value) -> Self {
        let extract_data = |s: &str| {
            user_data[s]
                .as_str()
                .expect("Error parsing user data.")
                .to_string()
        };

        let password = extract_data("password");

        Self {
            first_name: extract_data("first_name"),
            last_name: extract_data("last_name"),
            username: extract_data("username"),
            email: extract_data("email"),
            hashed_password: generate_hash(password),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLogin {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserFull {
    #[serde(flatten)]
    pub user: User,
    pub locations: Vec<Location>,
    pub reviews: Vec<Review>,
    pub images: Vec<Image>,
    pub owned_business: Vec<Business>,
}

#[derive(Debug, Clone)]
pub struct UserArray(Vec<User>);

impl UserArray {
    pub fn new(users: Vec<User>) -> Self {
        Self(users)
    }

    pub fn eager_load(&self, conn: &mut PgConnection) -> QueryResult<Value> {
        let result: Vec<Value> = self
            .0
            .clone()
            .into_iter()
            .filter_map(|r| r.get_result(conn).ok())
            .collect();

        Ok(serde_json::json!({
            "users": result
        }))
    }
}
