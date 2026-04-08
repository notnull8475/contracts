use crate::auth::auth;
use crate::services::contract_history as svc;
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
