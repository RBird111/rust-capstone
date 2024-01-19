use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use database::actions::business;
use database::models::business::*;
use serde_json::Value;

use crate::DBPool;

pub fn business_routes() -> actix_web::Scope {
    actix_web::web::scope("businesses")
        .service(get_all_businesses)
        .service(create_new_business)
        .service(get_business_by_id)
        .service(update_business)
        .service(delete_business)
}

#[get("")]
async fn get_all_businesses(state: DBPool, query: web::Query<Value>) -> Result<impl Responder> {
    let category = query.into_inner().get("category").map(|v| {
        v.as_str()
            .expect("error parsing query parameter")
            .to_string()
    });

    let businesses = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        business::get_all_businesses(&mut conn, category)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(businesses))
}

#[post("")]
async fn create_new_business(
    state: DBPool,
    data: web::Json<BusinessForm>,
) -> Result<impl Responder> {
    let business_data = data.into_inner();

    let business = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        business::create_new_business(&mut conn, business_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Created().json(business))
}

#[get("/{business_id}")]
async fn get_business_by_id(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let business_id = path.into_inner();

    let business = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        business::get_business_by_id(&mut conn, business_id)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(business))
}

#[put("/{business_id}")]
async fn update_business(state: DBPool, data: web::Json<Business>) -> Result<impl Responder> {
    let business_data = data.into_inner();

    let business = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        business::update_business(&mut conn, business_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(business))
}

#[delete("/{business_id}")]
async fn delete_business(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let business_id = path.into_inner();

    let message = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        business::delete_business(&mut conn, business_id)
    })
    .await?;

    Ok(HttpResponse::Ok().json(message))
}
