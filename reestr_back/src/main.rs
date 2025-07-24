mod auth;
mod handlers;
mod middleware;
mod models;
mod schema;
mod utils;
mod services;

use crate::auth::secure::login_user;
// use simple_logger::SimpleLogger;
use crate::utils::create_admin_user::create_admin_user_if_need;
use actix_cors::Cors;
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::fs::OpenOptions;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("image-cookie")) // arbitrary name
        .cookie_secure(false) // https only
        .session_lifecycle(BrowserSession::default()) // expire at end of session
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private) // encrypt
        .cookie_http_only(true) // disallow scripts from reading
        .build()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_admin_user_if_need();
    dotenv().ok();
    // Initialize the logger
    let _log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("application.log")
        .expect("Failed to open log file");

    // SimpleLogger::new()
    //     .with_level(LevelFilter::Debug)
    //     .init()
    //     .expect("Failed to initialize logger");

    info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(36000);
        /*
        API description
        guest
         - registration of person
        admin
         - add person
         - add specification of person
         - del person
         - del specification
         - list of persons
        person
         - login
         - logout
         - get table of it specifications
        */
        App::new()
            .wrap(session_middleware())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Встроенный логгер Actix
            .service(login_user)
            // GUEST ROLE
            .route(
                "/api/v1/registration",
                web::post().to(auth::secure::register_user),
            )
            // .route("/api/v1/login", web::post().to(services::secure::login_user))
            .route(
                "/api/v1/add/user",
                web::post().to(handlers::users::add_user),
            )
            .route("/api/v1/add/role", web::post()) //TODO
            .route(
                "/api/v1/update/user",
                web::post().to(handlers::users::update_user),
            )
            .route(
                "/api/v1/admin/user/update",
                web::post().to(handlers::users::update_user),
            )
            .route("/api/v1/update/role", web::post()) //TODO
            .route(
                "/api/v1/get/roles",
                web::get().to(handlers::users::get_roles),
            )
            .route(
                "/api/v1/get/users",
                web::get().to(handlers::users::get_users),
            )
            // DELETE
            .route(
                "/api/v1/admin/user/delete/{user_id}",
                web::delete().to(handlers::users::del_user),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
