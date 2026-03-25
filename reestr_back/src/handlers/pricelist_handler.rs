use crate::auth::auth;
use crate::models::pricelist_models::{PricelistInsertDTO, PricelistUpdateDTO};
use crate::services::pricelist as svc;
use crate::utils::utils::response_fn;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn list_req(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(r) = auth::verify_and_extract_claims(&req) {
        return Ok(r);
    }
    response_fn(svc::list_pricelist().await)
}

pub async fn add_req(
    req: HttpRequest,
    body: web::Json<PricelistInsertDTO>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(svc::add_pricelist(body.into_inner()).await)
}

pub async fn update_req(
    req: HttpRequest,
    body: web::Json<PricelistUpdateDTO>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(svc::update_pricelist(body.into_inner()).await)
}

pub async fn del_req(
    req: HttpRequest,
    pid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(r) = auth::check_admin_token(&req) {
        return Ok(r);
    }
    response_fn(svc::remove_pricelist(pid.into_inner()).await)
}
