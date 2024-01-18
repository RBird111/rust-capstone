use actix_web::error::ErrorNotFound;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use database::actions::image;
use database::models::image::*;

use crate::DBPool;

pub fn image_routes() -> actix_web::Scope {
    web::scope("/images")
        .service(get_all_images)
        .service(get_image_by_id)
        .service(upload_image)
        .service(delete_image)
}

#[get("")]
async fn get_all_images(state: DBPool) -> Result<impl Responder> {
    let images = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        image::get_all_images(&mut conn)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(images))
}

#[get("/{image_id}")]
async fn get_image_by_id(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let image_id = path.into_inner();

    let image = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        image::get_image_by_id(&mut conn, image_id)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(image))
}

#[post("")]
async fn upload_image(state: DBPool, data: web::Json<ImageForm>) -> Result<impl Responder> {
    let image_data = data.into_inner();

    let image = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        image::upload_image(&mut conn, image_data)
    })
    .await?
    .map_err(ErrorNotFound)?;

    Ok(HttpResponse::Created().json(image))
}

#[delete("/{image_id}")]
async fn delete_image(state: DBPool, path: web::Path<i32>) -> Result<impl Responder> {
    let image_id = path.into_inner();

    let message = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        image::delete_image(&mut conn, image_id)
    })
    .await?;

    Ok(HttpResponse::Ok().json(message))
}

// #[get("/curr")]
// async fn get_user_images(state: DBPool) -> Result<impl Responder> {
//     todo!()
// }
