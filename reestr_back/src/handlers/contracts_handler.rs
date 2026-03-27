use crate::auth::auth;
use crate::models::contract_models::{Contract, ContractDTO, ContractListParams, ContractStatsResponse, PaginatedContractsResponse};
use crate::services::contract::{
    add_contract, batch_stats, get_contract, list_contract, list_contract_paginated,
    remove_contract, update_contract,
};
use crate::utils::utils::response_fn;
use actix_web::{Error, HttpRequest, HttpResponse, web};

pub async fn add_contract_req(
    req: HttpRequest,
    contract: web::Json<ContractDTO>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Contract, String> = add_contract(contract.into_inner()).await;
    response_fn(resp)
}

pub async fn del_contract_req(
    req: HttpRequest,
    contract_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<usize, String> = remove_contract(contract_id.into_inner()).await;
    response_fn(resp)
}

pub async fn update_contract_req(
    req: HttpRequest,
    contract: web::Json<Contract>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp = update_contract(contract.into_inner()).await;
    response_fn(resp)
}

pub async fn list_contract_req(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Vec<Contract>, String> = list_contract().await;
    response_fn(resp)
}

/// Пагинированный список с фильтрами
pub async fn list_paginated_req(
    req: HttpRequest,
    params: web::Query<ContractListParams>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp = list_contract_paginated(params.into_inner()).await;
    response_fn(resp)
}

/// Batch счётчики файлов и доп соглашений
pub async fn batch_stats_req(req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp = batch_stats().await;
    response_fn(resp)
}

pub async fn get_contract_req(
    req: HttpRequest,
    contract_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Err(response) = auth::verify_and_extract_claims(&req) {
        return Ok(response);
    }
    let resp: Result<Contract, String> = get_contract(contract_id.into_inner()).await;
    response_fn(resp)
}
