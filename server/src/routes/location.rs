use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use database::actions::location;
use database::models::location::*;

use crate::DBPool;

pub fn location_routes() -> actix_web::Scope {
    actix_web::web::scope("/locations")
        .service(get_all_locations)
        .service(get_location_by_id)
        .service(create_new_location)
        .service(update_location)
        .service(delete_location)
}

#[get("")]
async fn get_all_locations(state: DBPool) -> Result<impl Responder> {
    let locations = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        location::get_all_locations(&mut conn)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(locations))
}

#[get("/{location_id}")]
async fn get_location_by_id(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let location_id = path.into_inner();

    let location = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        location::get_location_by_id(&mut conn, location_id)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(location))
}

#[post("")]
async fn create_new_location(
    state: DBPool,
    data: web::Json<LocationForm>,
) -> Result<impl Responder> {
    let location_data = data.into_inner();

    let location = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        location::create_new_location(&mut conn, location_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Created().json(location))
}

#[put("/{location_id}")]
async fn update_location(
    state: DBPool,
    path: web::Path<i32>,
    data: web::Json<Location>,
) -> Result<impl Responder> {
    let _location_id = path.into_inner();
    let location_data = data.into_inner();

    let location = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        location::update_location(&mut conn, location_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(location))
}

#[delete("/{location_id}")]
async fn delete_location(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let location_id = path.into_inner();

    let message = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        location::delete_location(&mut conn, location_id)
    })
    .await?;

    Ok(HttpResponse::Ok().json(message))
}
