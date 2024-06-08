pub mod routes;

use actix_files::{Files, NamedFile};
use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{Key, SameSite},
    dev, middleware, web, App, HttpServer,
};
use database::ConnectionPool;

pub type DBPool = web::Data<ConnectionPool>;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name("test-cookie".to_string())
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

fn default() -> Files {
    Files::new("/", "./frontend/build")
        .index_file("index.html")
        .default_handler(|req: dev::ServiceRequest| {
            let (http_req, _payload) = req.into_parts();
            async {
                let response =
                    NamedFile::open("./frontend/build/index.html")?.into_response(&http_req);
                Ok(dev::ServiceResponse::new(http_req, response))
            }
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    let port: u16 = std::env::var("PORT")
        .unwrap_or("4000".to_string())
        .parse()
        .expect("PORT must be an integer");

    let host = match std::env::var("RUST_ENV") {
        Ok(val) => match val.as_str() {
            "production" => "0.0.0.0",
            "development" => "127.0.1",
            _ => unreachable!(),
        },
        Err(_) => "0.0.0.0",
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::get_pool();

    log::info!("Listening in on port {port}...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .wrap(session_middleware())
            .app_data(web::Data::new(pool.clone()))
            .service(routes::api_routes())
            .service(default())
    })
    .bind((host, port))?
    .run()
    .await
}
