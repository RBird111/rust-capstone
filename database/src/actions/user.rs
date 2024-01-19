use super::DataResult;
use crate::models::user::*;
use crate::schema::users::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use password_auth::verify_password;
use serde_json::Value;

pub fn get_all_users(conn: &mut PgConnection) -> DataResult<Value> {
    let all_users: Vec<User> = users.select(User::as_select()).load(conn)?;
    Ok(UserArray::new(all_users).eager_load(conn)?)
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
    let new_user: User = diesel::insert_into(users)
        .values(user_data)
        .returning(User::as_returning())
        .get_result(conn)?;

    Ok(new_user)
}

pub fn update_user(conn: &mut PgConnection, user: User) -> DataResult<Value> {
    let updated_user: User = diesel::update(users.filter(id.eq(user.id)))
        .set(user)
        .get_result(conn)?;
    Ok(updated_user.eager_load(conn)?)
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
