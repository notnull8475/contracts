use crate::auth::auth;
use crate::models::models::{Organization, OrganizationDTO};
use crate::services::organization::{
    add_organization, get_organization, list_organization, remove_organization, update_organization,
};
use crate::utils::utils::response_fn;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn add_organization_req(
    req: HttpRequest,
    organization: web::Json<OrganizationDTO>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Organization, String> = add_organization(organization.into_inner()).await;
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

pub async fn organizations_list(req: HttpRequest) -> Result<HttpResponse, Error> {
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
