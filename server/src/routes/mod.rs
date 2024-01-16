pub mod auth;
pub mod user;

pub fn api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api")
        .service(auth::auth_routes())
        .service(user::user_routes())
}
