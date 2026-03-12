use actix_web::{web, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::io::Write;

use crate::services::contract_files as file_service;

pub async fn upload_file(
    mut payload: Multipart,
    contract_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let contract_id = contract_id.into_inner();

    while let Some(item) = payload.try_next().await.map_err(|e| {
        actix_web::error::ErrorBadRequest(e.to_string())
    })? {
        let content_disposition = item.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let mime_type = item
            .content_type()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let mut file_data = Vec::new();
        let mut field = item;
        while let Some(chunk) = field.try_next().await.map_err(|e| {
            actix_web::error::ErrorBadRequest(e.to_string())
        })? {
            file_data.write_all(&chunk).map_err(|e| {
                actix_web::error::ErrorBadRequest(e.to_string())
            })?;
        }

        match file_service::save_file(contract_id, file_data, filename, mime_type).await {
            Ok(file) => {
                return Ok(HttpResponse::Ok().json(file));
            }
            Err(e) => {
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": e
                })));
            }
        }
    }

    Ok(HttpResponse::BadRequest().json(serde_json::json!({
        "error": "No file provided"
    })))
}

pub async fn get_contract_files(
    contract_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let contract_id = contract_id.into_inner();

    match file_service::get_files_by_contract(contract_id).await {
        Ok(files) => Ok(HttpResponse::Ok().json(files)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        }))),
    }
}

pub async fn download_file(
    file_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let file_id = file_id.into_inner();

    match file_service::get_file_by_id(file_id).await {
        Ok(file) => {
            let file_path = file_service::get_files_dir().join(&file.file_name);
            match std::fs::read(&file_path) {
                Ok(data) => {
                    let mut response = HttpResponse::Ok();
                    response.insert_header(
                        actix_web::http::header::ContentDisposition::attachment(&file.orig_name)
                    );
                    response.insert_header((
                        actix_web::http::header::CONTENT_TYPE,
                        file.mime_type_txt.clone(),
                    ));
                    response.insert_header((
                        actix_web::http::header::CONTENT_LENGTH,
                        file.size_bytes.to_string(),
                    ));
                    Ok(response.body(data))
                }
                Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Failed to read file: {}", e)
                }))),
            }
        }
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": e
        }))),
    }
}

pub async fn delete_file(
    file_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let file_id = file_id.into_inner();

    match file_service::delete_file(file_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "File deleted"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        }))),
    }
}
