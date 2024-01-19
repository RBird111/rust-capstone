pub mod routes;

use actix_files::{Files, NamedFile};
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};
use actix_web::{dev, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use database::ConnectionPool;

pub type DBPool = web::Data<ConnectionPool>;

#[derive(Debug, serde::Deserialize)]
struct CookieModel {
    message: String,
}

#[get("get_session")]
async fn get_session(session: Session) -> impl Responder {
    match session.get::<String>("message") {
        Ok(message_option) => match message_option {
            Some(message) => HttpResponse::Ok().body(message),
            None => HttpResponse::NotFound().body("Not set."),
        },
        Err(_) => HttpResponse::InternalServerError().body("Session error."),
    }
}

#[post("set_session")]
async fn set_session(session: Session, model: web::Json<CookieModel>) -> impl Responder {
    match session.insert("message", model.message.clone()) {
        Ok(_) => HttpResponse::Created().body("Created."),
        Err(_) => HttpResponse::InternalServerError().body("Error."),
    }
}

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
    dotenvy::dotenv().expect(".env not found");

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be an integer");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::get_pool();

    log::info!("Listening in on port {port}...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .wrap(session_middleware())
            .app_data(web::Data::new(pool.clone()))
            .service(get_session)
            .service(set_session)
            .service(routes::api_routes())
            .service(default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
