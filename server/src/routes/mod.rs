pub mod auth;
pub mod business;
pub mod location;
pub mod review;
pub mod user;

pub fn api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api")
        .service(auth::auth_routes())
        .service(user::user_routes())
        .service(location::location_routes())
        .service(business::business_routes())
        .service(review::review_routes())
}
