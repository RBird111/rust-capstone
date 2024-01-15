pub mod users;

pub fn api_routes() -> actix_web::Scope {
    actix_web::web::scope("/api").service(users::user_routes())
}
