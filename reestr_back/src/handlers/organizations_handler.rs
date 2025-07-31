use crate::auth::auth;
use crate::models::organization_models::{NewOrganizationDTO, Organization};
use crate::services::organization::{
    add_organization, get_organization, list_organization, remove_organization, update_organization,
};
use crate::utils::utils::response_fn;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn add_organization_req(
    req: HttpRequest,
    organization: web::Json<NewOrganizationDTO>,
) -> Result<HttpResponse, Error> {
    info!("запрос на добавление организации {:?}", req);
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let organization = match organization.validate_and_convert() {
        Ok(org) => org,
        Err(err) => {
            return Ok(HttpResponse::BadRequest().body(format!("Validation error: {}", err)));
        }
    };
    let resp: Result<Organization, String> = add_organization(organization).await;
    response_fn(resp)
}
pub async fn del_organization_req(
    req: HttpRequest,
    organization_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<usize, String> = remove_organization(organization_id.into_inner()).await;
    response_fn(resp)
}

pub async fn update_organization_req(
    req: HttpRequest,
    organization: web::Json<Organization>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp = update_organization(organization.into_inner()).await;
    response_fn(resp)
}

pub async fn list_organization_req(req: HttpRequest) -> impl Responder {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Vec<Organization>, String> = list_organization().await;
    response_fn(resp)
}
pub async fn get_organization_req(
    req: HttpRequest,
    organization_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Organization, String> = get_organization(organization_id.into_inner()).await;
    response_fn(resp)
}
