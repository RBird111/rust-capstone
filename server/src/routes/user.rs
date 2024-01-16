use actix_web::error::ErrorInternalServerError;
use actix_web::{delete, get, put, web, HttpResponse, Responder};
use database::actions::user;
use database::models::user::User;

use crate::DBPool;

pub fn user_routes() -> actix_web::Scope {
    actix_web::web::scope("/user")
        .service(get_all_users)
        .service(get_user_by_id)
        .service(update_user)
        .service(delete_user)
}

#[get("")]
async fn get_all_users(state: DBPool) -> actix_web::Result<impl Responder> {
    let users = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::get_all_users(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/{user_id}")]
async fn get_user_by_id(state: DBPool, path: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let user_id = path.into_inner();

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::get_user_by_id(&mut conn, user_id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/curr")]
async fn update_user(state: DBPool, data: web::Json<User>) -> actix_web::Result<impl Responder> {
    let user_data = data.into_inner();

    let updated_user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::update_user(&mut conn, user_data)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_user))
}

#[delete("/curr")]
async fn delete_user(state: DBPool, data: web::Json<User>) -> actix_web::Result<impl Responder> {
    let user = data.into_inner();

    let msg = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user::delete_user(&mut conn, user.id)
    })
    .await?;

    Ok(HttpResponse::Ok().json(msg))
}
