use crate::auth::auth;
use crate::models::contract_status_models::{ContractStatusInsertDTO, ContractStatusUpdateDTO};
use crate::services::contract_status::{add_status, list_statuses, remove_status, update_status};
use crate::utils::utils::response_fn;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn list_statuses_req(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(list_statuses().await)
}

pub async fn add_status_req(
    req: HttpRequest,
    body: web::Json<ContractStatusInsertDTO>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(add_status(body.into_inner()).await)
}

pub async fn update_status_req(
    req: HttpRequest,
    body: web::Json<ContractStatusUpdateDTO>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(update_status(body.into_inner()).await)
}

pub async fn del_status_req(
    req: HttpRequest,
    status_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(remove_status(status_id.into_inner()).await)
}
