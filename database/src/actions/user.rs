use super::DataResult;
use crate::models::user::{User, UserForm, UserLogin};
use crate::schema::users::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use password_auth::verify_password;
use serde_json::Value;

pub fn get_all_users(conn: &mut PgConnection) -> DataResult<Vec<User>> {
    Ok(users.select(User::as_select()).load(conn)?)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> DataResult<User> {
    Ok(users.find(user_id).first(conn)?)
}

pub fn login(conn: &mut PgConnection, login: UserLogin) -> DataResult<User> {
    let UserLogin {
        credential,
        password,
    } = login;

    let user: User = users
        .filter(username.eq(&credential))
        .or_filter(email.eq(&credential))
        .select(User::as_select())
        .first(conn)?;

    verify_password(password, &user.hashed_password)
        .map_err(|_| diesel::result::Error::NotFound)?;

    Ok(user)
}

pub fn create_new_user(conn: &mut PgConnection, user_data: UserForm) -> DataResult<User> {
    Ok(diesel::insert_into(users)
        .values(user_data)
        .returning(User::as_returning())
        .get_result(conn)?)
}

pub fn update_user(conn: &mut PgConnection, user: User) -> DataResult<User> {
    Ok(diesel::update(users).set(user).get_result(conn)?)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Value {
    let success = r#"{"message": "User successfully deleted"}"#;
    let error = r#"{"message": "Unable to locate user"}"#;

    match diesel::delete(users.find(user_id)).execute(conn) {
        Ok(0) => serde_json::from_str(error),
        Ok(_) => serde_json::from_str(success),
        Err(_) => serde_json::from_str(error),
    }
    .expect("error serializing message")
}
