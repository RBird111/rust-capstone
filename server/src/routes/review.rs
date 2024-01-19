use super::get_authenticated_user;
use crate::DBPool;

use actix_session::Session;
use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use database::actions::review;
use database::models::review::*;
use serde_json::Value;

pub fn review_routes() -> actix_web::Scope {
    web::scope("/reviews")
        .service(get_all_reviews)
        .service(get_review_by_id)
        .service(get_random_reviews)
        .service(create_new_review)
        .service(update_review)
        .service(delete_review)
        .service(get_user_reviews)
}

#[get("")]
async fn get_all_reviews(state: DBPool) -> Result<impl Responder> {
    let reviews = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::get_all_reviews(&mut conn)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(reviews))
}

#[get("/{review_id}")]
async fn get_review_by_id(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let review_id = path.into_inner();

    let review = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::get_review_by_id(&mut conn, review_id)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(review))
}

#[get("/random/{num}")]
async fn get_random_reviews(state: DBPool, path: web::Path<usize>) -> Result<impl Responder> {
    let num = path.into_inner();

    let reviews = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::get_random_reviews(&mut conn, num)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(reviews))
}

#[post("")]
async fn create_new_review(
    session: Session,
    state: DBPool,
    data: web::Json<Value>,
) -> Result<impl Responder> {
    let user = match get_authenticated_user(&session, state.clone()).await {
        Some(u) => u,
        None => return Ok(HttpResponse::Ok().json(serde_json::json!({"errors": ["Unauthorized"]}))),
    };

    let form_data = data.into_inner();

    let review = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::create_new_review(&mut conn, form_data, user)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Created().json(review))
}

#[put("/{review_id}")]
async fn update_review(state: DBPool, data: web::Json<Review>) -> Result<impl Responder> {
    let review_data = data.into_inner();

    let review = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::update_review(&mut conn, review_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(review))
}

#[delete("/{review_id}")]
async fn delete_review(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let review_id = path.into_inner();

    let message = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::delete_review(&mut conn, review_id)
    })
    .await?;

    Ok(HttpResponse::Ok().json(message))
}

#[get("/user/curr")]
async fn get_user_reviews(session: Session, state: DBPool) -> Result<impl Responder> {
    let user = match get_authenticated_user(&session, state.clone()).await {
        Some(u) => u,
        None => return Ok(HttpResponse::Ok().json(serde_json::json!({"errors": ["Unauthorized"]}))),
    };

    let reviews = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        review::get_user_reviews(&mut conn, user)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(reviews))
}
