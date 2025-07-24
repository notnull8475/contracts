mod auth;
mod handlers;
mod middleware;
mod models;
mod schema;
mod services;
mod utils;

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
        App::new()
            .wrap(session_middleware())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Встроенный логгер Actix
            .service(
                web::scope("/api/v1")
                    .route("/registration", web::post().to(auth::secure::register_user))
            .route("/login", web::post().to(auth::secure::login_user))
            .route("/roles/get", web::get().to(handlers::users::get_roles))
            .service(
                web::scope("/admin")
                    .service(
                        web::scope("/users")
                            .route("/update", web::post().to(handlers::users::update_user))
                            .route("/add", web::post().to(handlers::users::add_user))
                            .route("/delete/{user_id}",web::delete().to(handlers::users::del_user))
                            .route("/get/list", web::get().to(handlers::users::get_users)),
                    )
                )
                .service(
                    web::scope("/types/validity")
                        .route("/add",web::post().to(handlers::type_of_validity_handler::add_type_req))
                        .route("/del/{type_id}",web::delete().to(handlers::type_of_validity_handler::del_type_req))
                        .route("/list",web::get().to(handlers::type_of_validity_handler::types_list))
                        .route("/get/{type_id}",web::get().to(handlers::type_of_validity_handler::get_type_req))
                 )
                .service(
                    web::scope("/contracts")
                        .route("/add", web::post().to(handlers::contracts_handler::add_contract_req))
                        .route("/update", web::post().to(handlers::contracts_handler::update_contract_req))
                        .route("/del/{id}", web::delete().to(handlers::contracts_handler::del_contract_req))
                        .route("/list", web::get().to(handlers::contracts_handler::list_contract_req))
                        .route("/get/{id}", web::get().to(handlers::contracts_handler::get_contract_req)),
                )
                .service(
                    web::scope("/organizations")
                        .route("/add", web::post().to(handlers::organizations_handler::add_organization_req))
                        .route("/update", web::post().to(handlers::organizations_handler::update_organization_req))
                        .route("/del/{id}", web::delete().to(handlers::organizations_handler::del_organization_req))
                        .route("/list", web::get().to(handlers::organizations_handler::list_organization_req))
                        .route("/get/{id}", web::get().to(handlers::organizations_handler::get_organization_req)),
                )
                .service(
                    web::scope("/responsible_person")
                        .route("/add", web::post().to(handlers::responsible_person_handler::add_responsible_person_req))
                        .route("/update", web::post().to(handlers::responsible_person_handler::update_responsible_person_req))
                        .route("/del/{id}", web::delete().to(handlers::responsible_person_handler::del_responsible_person_req))
                        .route("/list", web::get().to(handlers::responsible_person_handler::list_responsible_person_req))
                        .route("/get/{id}", web::get().to(handlers::responsible_person_handler::get_responsible_person_req)),
                )
            )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
