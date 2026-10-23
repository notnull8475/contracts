mod auth;
mod conf;
mod handlers;
mod models;
mod schema;
mod services;
mod utils;

use crate::conf::conf::init_env;
use crate::services::contract_files;
use crate::utils::create_admin_user::create_admin_user_if_need;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

// Аутентификация только по JWT в заголовке Authorization: cookie-сессия actix
// (`image-cookie`) ни одним обработчиком не читалась и была удалена.
const BIND_ADDR: &str = "127.0.0.1:8080";

/// Ошибки разбора JSON отдаём как `{"error": "..."}` — этот формат читает фронт.
/// По умолчанию actix отвечает простым текстом, и пользователь видел лишь общее
/// «Ошибка сохранения» без причины.
fn json_config() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| {
        let message = format!("Некорректные данные запроса: {}", err);
        log::warn!("{}", message);
        actix_web::error::InternalError::from_response(
            err,
            actix_web::HttpResponse::BadRequest().json(serde_json::json!({ "error": message })),
        )
        .into()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();
    create_admin_user_if_need();
    
    if let Err(e) = contract_files::init_files_dir() {
        eprintln!("Failed to init files directory: {}", e);
    }
    
    // Логи идут в stdout → journald (journalctl -u reestr-back).
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .expect("Failed to initialize logger");

    // Только loopback: снаружи API доступен через nginx (/api/ → 127.0.0.1:8080),
    // прямой доступ к :8080 из сети обходил бы его лимиты и логи.
    info!("Starting server at http://{}", BIND_ADDR);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(36000);
        App::new()
            .app_data(json_config())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Встроенный логгер Actix
            .service(
                web::scope("/api/v1")
                    // Публичной регистрации в системе нет: пользователей заводит администратор
                    // через /api/v1/admin/users/add.
                    .route("/login", web::post().to(auth::secure::login_user))
                    .route("/roles/get", web::get().to(handlers::users::get_roles))
                    .service(
                        web::scope("/admin")
                            .service(
                                web::scope("/users")
                                    .route("/update", web::post().to(handlers::users::update_user))
                                    .route("/add", web::post().to(handlers::users::add_user))
                                    .route("/delete/{user_id}", web::delete().to(handlers::users::del_user))
                                    .route("/get/list", web::get().to(handlers::users::get_users)),
                            )
                            .service(
                                web::scope("/contract-statuses")
                                    .route("/list", web::get().to(handlers::contract_status_handler::list_statuses_req))
                                    .route("/add", web::post().to(handlers::contract_status_handler::add_status_req))
                                    .route("/update", web::post().to(handlers::contract_status_handler::update_status_req))
                                    .route("/del/{status_id}", web::delete().to(handlers::contract_status_handler::del_status_req)),
                            )
                            .service(
                                web::scope("/pricelist")
                                    .route("/list", web::get().to(handlers::pricelist_handler::list_req))
                                    .route("/add", web::post().to(handlers::pricelist_handler::add_req))
                                    .route("/update", web::post().to(handlers::pricelist_handler::update_req))
                                    .route("/del/{id}", web::delete().to(handlers::pricelist_handler::del_req)),
                            )
                    )
                    .service(
                        web::scope("/types/validity")
                            .route("/add", web::post().to(handlers::type_of_validity_handler::add_type_req))
                            .route("/del/{type_id}", web::delete().to(handlers::type_of_validity_handler::del_type_req))
                            .route("/list", web::get().to(handlers::type_of_validity_handler::types_list))
                            .route("/get/{type_id}", web::get().to(handlers::type_of_validity_handler::get_type_req))
                    )
.service(
                        web::scope("/contracts")
                            .route("/add", web::post().to(handlers::contracts_handler::add_contract_req))
                            .route("/update", web::post().to(handlers::contracts_handler::update_contract_req))
                            .route("/del/{id}", web::delete().to(handlers::contracts_handler::del_contract_req))
                            .route("/list", web::get().to(handlers::contracts_handler::list_contract_req))
                            .route("/paginated", web::get().to(handlers::contracts_handler::list_paginated_req))
                            .route("/stats", web::get().to(handlers::contracts_handler::batch_stats_req))
                            .route("/summary", web::get().to(handlers::contracts_handler::summary_req))
                            .route("/get/{id}", web::get().to(handlers::contracts_handler::get_contract_req))
                            .route("/files/{contract_id}", web::post().to(handlers::contract_files_handler::upload_file))
                            .route("/files/{contract_id}", web::get().to(handlers::contract_files_handler::get_contract_files))
                            .route("/files/download/{file_id}", web::get().to(handlers::contract_files_handler::download_file))
                            .route("/files/delete/{file_id}", web::delete().to(handlers::contract_files_handler::delete_file))
                            .route("/supplementary-agreements/{contract_id}", web::get().to(handlers::supplementary_agreement_handler::list_by_contract_req))
                            .route("/supplementary-agreements/count/{contract_id}", web::get().to(handlers::supplementary_agreement_handler::count_by_contract_req))
                            .route("/supplementary-agreements/add", web::post().to(handlers::supplementary_agreement_handler::add_req))
                            .route("/supplementary-agreements/update", web::post().to(handlers::supplementary_agreement_handler::update_req))
                            .route("/supplementary-agreements/del/{id}", web::delete().to(handlers::supplementary_agreement_handler::del_req))
                            .route("/supplementary-agreements/files/{sa_id}", web::get().to(handlers::contract_files_handler::get_sa_files))
                            .route("/history/{contract_id}", web::get().to(handlers::contract_history_handler::list_by_contract_req))
                    )
                    .service(
                        web::scope("/organizations")
                            .route("/add", web::post().to(handlers::organizations_handler::add_organization_req))
                            .route("/update", web::post().to(handlers::organizations_handler::update_organization_req))
                            .route("/del/{id}", web::delete().to(handlers::organizations_handler::del_organization_req))
                            .route("/list", web::get().to(handlers::organizations_handler::list_organization_req))
                            .route("/get/{id}", web::get().to(handlers::organizations_handler::get_organization_req))
                            .route("/get_by_inn/{inn}", web::get().to(handlers::dadata_handler::get_org_by_inn))
                    )
                    .service(
                        web::scope("/responsible_person")
                            .route("/add", web::post().to(handlers::responsible_person_handler::add_responsible_person_req))
                            .route("/update", web::post().to(handlers::responsible_person_handler::update_responsible_person_req))
                            .route("/del/{id}", web::delete().to(handlers::responsible_person_handler::del_responsible_person_req))
                            .route("/list", web::get().to(handlers::responsible_person_handler::list_responsible_person_req))
                            .route("/get/{id}", web::get().to(handlers::responsible_person_handler::get_responsible_person_req))
                    )
            )
    })
        .bind(BIND_ADDR)?
        .run()
        .await
}
