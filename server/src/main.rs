pub mod routes;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/")]
async fn home() -> impl Responder {
    let index = r#"
    <head>
        <title>Test Title</title>
    </head>

    <h1>Hello World!</h1>

    <p>This is a test p element</p>

    <button style="background-color: blue">Test Button</button>
    "#;

    HttpResponse::Ok().body(index)
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
            .service(routes::api_routes())
    })
    .bind(("localhost", port))?
    .run()
    .await
}
