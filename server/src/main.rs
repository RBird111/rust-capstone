pub mod routes;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};
use database::models::business::{BusinessData, BusinessForm};
use database::models::location::LocationForm;

#[actix_web::get("/")]
async fn home() -> impl Responder {
    let index = r#"
    <head>
        <title>Test Title</title>
    </head>

    <h1>Hello World!</h1>

    <p>This is a test sentence.</p>

    <a style="display: block" href="/json">Test Link</a>

    <br/>

    <button style="background-color: blue">Test Button</button>
    "#;

    HttpResponse::Ok().body(index)
}

#[actix_web::get("/json")]
async fn test_json() -> impl Responder {
    let business = BusinessData {
        name: "Test Business".to_string(),
        description: "abcdefghijklmnopqrstuvwxyz".to_string(),
        category: "Automotive".to_string(),
        owner_id: None,
    };

    let location = LocationForm {
        address: "4241 Test St".to_string(),
        city: "Albuquerque".to_string(),
        state: "NM".to_string(),
    };

    let test = BusinessForm { business, location };

    HttpResponse::Ok().json(test)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env not found");

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be an int");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::get_pool();

    log::info!("Listening in on port {port}...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .app_data(Data::new(pool.clone()))
            .service(home)
            .service(test_json)
            .service(routes::api_routes())
    })
    .bind(("localhost", port))?
    .run()
    .await
}
