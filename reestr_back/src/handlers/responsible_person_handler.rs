use crate::auth::auth;
use crate::models::models::{ResponsiblePerson, ResponsiblePersonDTO};
use crate::services::responsible_person::{
    add_responsible_person, get_responsible_person, list_responsible_person,
    remove_responsible_person, update_responsible_person,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use crate::utils::utils::response_fn;

pub async fn add_responsible_person_req(
    req: HttpRequest,
    responsible_person: web::Json<ResponsiblePersonDTO>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<ResponsiblePerson, String> =
        add_responsible_person(responsible_person.into_inner()).await;
    response_fn(resp)
}
pub async fn del_responsible_person_req(
    req: HttpRequest,
    responsible_person_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<usize, String> =
        remove_responsible_person(responsible_person_id.into_inner()).await;
    response_fn(resp)
}

pub async fn update_responsible_person_req(
    req: HttpRequest,
    responsible_person: web::Json<ResponsiblePerson>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp = update_responsible_person(responsible_person.into_inner()).await;
    response_fn(resp)
}

pub async fn list_responsible_person_req(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Vec<ResponsiblePerson>, String> = list_responsible_person().await;
    response_fn(resp)
}
pub async fn get_responsible_person_req(
    req: HttpRequest,
    responsible_person_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<ResponsiblePerson, String> =
        get_responsible_person(responsible_person_id.into_inner()).await;
    response_fn(resp)
}
