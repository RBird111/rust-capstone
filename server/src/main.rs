pub mod routes;

use actix_files as fs;
use actix_web::web;
use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};
use database::ConnectionPool;

pub type DBPool = web::Data<ConnectionPool>;

#[actix_web::get("/")]
async fn home() -> impl Responder {
    let index = include_str!("../../frontend/build/index.html");
    HttpResponse::Ok().body(index)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env not found");

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be an integer");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::get_pool();

    log::info!("Listening in on port {port}...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .app_data(web::Data::new(pool.clone()))
            .service(home)
            .service(routes::api_routes())
            .service(fs::Files::new("/", "./frontend/build").show_files_listing())
    })
    .bind(("localhost", port))?
    .run()
    .await
}
