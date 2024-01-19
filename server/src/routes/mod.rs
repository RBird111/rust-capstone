pub mod auth;
pub mod business;
pub mod image;
pub mod location;
pub mod review;
pub mod user;

use crate::DBPool;

use actix_session::Session;
use actix_web::web;
use database::actions::user as user_actions;
use database::models::user::User;

pub fn api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api")
        .service(auth::auth_routes())
        .service(user::user_routes())
        .service(location::location_routes())
        .service(business::business_routes())
        .service(review::review_routes())
        .service(image::image_routes())
}

pub async fn get_authenticated_user(session: &Session, state: DBPool) -> Option<User> {
    let user_id: i32 = match session.get("user_id") {
        Ok(id_option) => match id_option {
            Some(id) => id,
            None => return None,
        },
        Err(_) => return None,
    };

    let user = web::block(move || {
        let mut conn = state.get().expect("error connecting to database");
        user_actions::get_user_by_id(&mut conn, user_id)
    })
    .await
    .ok()?;

    user.ok()
}
