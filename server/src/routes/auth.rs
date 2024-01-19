use super::get_authenticated_user;
use crate::DBPool;

use actix_session::Session;
use actix_web::error::ErrorNotFound;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use database::actions::user;
use database::models::user::{UserForm, UserLogin};
use serde_json::{json, Value};

pub fn auth_routes() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(authenticate)
        .service(login)
        .service(signup)
        .service(logout)
}

#[get("")]
async fn authenticate(session: Session, state: DBPool) -> Result<impl Responder> {
    let user = match get_authenticated_user(&session, state.clone()).await {
        Some(u) => u,
        None => return Ok(HttpResponse::Ok().json(json!({"errors": ["Unauthorized"]}))),
    };

    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(
    session: Session,
    state: DBPool,
    data: web::Json<UserLogin>,
) -> Result<impl Responder> {
    let user_data = data.into_inner();

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::login(&mut conn, user_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    session.insert("user_id", user.id)?;
    session.renew();

    Ok(HttpResponse::Ok().json(user))
}

#[post("/signup")]
async fn signup(session: Session, state: DBPool, data: web::Json<Value>) -> Result<impl Responder> {
    let user_data = UserForm::from_json(data.into_inner());

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::create_new_user(&mut conn, user_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    session.insert("user_id", user.id)?;
    session.renew();

    Ok(HttpResponse::Created().json(user))
}

#[get("/logout")]
async fn logout(session: Session, _state: DBPool) -> Result<impl Responder> {
    session.purge();
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .body("User logged out."))
}
