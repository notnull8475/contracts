use crate::auth::auth;
use crate::models::models::{NewType, TypeOfValidity};
use crate::services::type_of_validity::{add_type, get_type, list_type, remove_type};
use crate::utils::utils::response_fn;
use actix_web::{Error, HttpRequest, HttpResponse, web};

pub async fn add_type_req(
    req: HttpRequest,
    type_name: web::Json<NewType>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<TypeOfValidity, String> = add_type(String::to_string(&type_name.name)).await;
    response_fn(resp)
}
pub async fn del_type_req(
    req: HttpRequest,
    type_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<usize, String> = remove_type(type_id.into_inner()).await;
    response_fn(resp)
}

pub async fn types_list(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Vec<TypeOfValidity>, String> = list_type().await;
    response_fn(resp)
}
pub async fn get_type_req(
    req: HttpRequest,
    type_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<TypeOfValidity, String> = get_type(type_id.into_inner()).await;
    response_fn(resp)
}
