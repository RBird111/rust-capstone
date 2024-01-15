use crate::schema::users;

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

#[derive(Insertable, AsChangeset)]
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
