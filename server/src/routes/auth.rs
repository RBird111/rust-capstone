use actix_web::error::ErrorNotFound;
use actix_web::{post, web, HttpResponse, Responder, Result};
use database::actions::user;
use database::models::user::{UserForm, UserLogin};
use serde_json::Value;

use crate::DBPool;

pub fn auth_routes() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(login)
        .service(signup)
        .service(logout)
}

#[post("/login")]
async fn login(state: DBPool, data: web::Json<UserLogin>) -> Result<impl Responder> {
    let user_data = data.into_inner();

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::login(&mut conn, user_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/signup")]
async fn signup(state: DBPool, data: web::Json<Value>) -> Result<impl Responder> {
    let user_data = UserForm::from_json(data.into_inner());

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::create_new_user(&mut conn, user_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Created().json(user))
}

#[post("/logout")]
async fn logout(_state: DBPool) -> Result<impl Responder> {
    // TODO: Deal with logout
    // let success: &str = r#"{"message": "User logged out"}"#;
    // let error: &str = r#"{"message": "Unable to locate user"}"#;
    Ok(HttpResponse::Ok())
}
