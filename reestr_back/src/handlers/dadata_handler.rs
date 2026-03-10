use crate::auth::auth;
use crate::utils::dadata;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use serde_json::json;

pub async fn get_org_by_inn(
    req: HttpRequest,
    inn: web::Path<String>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }

    println!("ИНН {}", &inn);
    let resp = dadata::find_party(&inn).await?;

    if let Some(org_info) = resp.extract_first_organization() {
        Ok(HttpResponse::Ok().json(org_info))
    } else {
        Ok(HttpResponse::build(StatusCode::NOT_FOUND)
            .json(json!({"error": "Organization not found"})))
    }
}
