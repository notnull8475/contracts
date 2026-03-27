use crate::auth::auth;
use crate::models::supplementary_agreement_models::{SupplementaryAgreementDTO, SupplementaryAgreementUpdateDTO};
use crate::services::supplementary_agreement as svc;
use crate::utils::utils::response_fn;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn list_by_contract_req(
    req: HttpRequest,
    cid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(svc::list_by_contract(cid.into_inner()).await)
}

pub async fn add_req(
    req: HttpRequest,
    body: web::Json<SupplementaryAgreementDTO>,
) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(svc::add(body.into_inner()).await)
}

pub async fn update_req(
    req: HttpRequest,
    body: web::Json<SupplementaryAgreementUpdateDTO>,
) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(svc::update(body.into_inner()).await)
}

pub async fn del_req(
    req: HttpRequest,
    sid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(svc::remove(sid.into_inner()).await)
}

pub async fn count_by_contract_req(
    req: HttpRequest,
    cid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    let result = svc::count_by_contract(cid.into_inner()).await;
    match result {
        Ok(count) => Ok(HttpResponse::Ok().json(serde_json::json!({ "count": count }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e }))),
    }
}
