use crate::models::contract_file_models::{ContractFile, ContractFileDTO, ContractFileResponse};
use crate::schema::contract_files::dsl::*;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

const FILES_DIR: &str = "./files";

pub fn init_files_dir() -> std::io::Result<()> {
    let path = PathBuf::from(FILES_DIR);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_files_dir() -> PathBuf {
    PathBuf::from(FILES_DIR)
}

pub async fn save_file(
    contract_id: i32,
    file_data: Vec<u8>,
    original_name: String,
    mime_type: String,
    ftype: String,
) -> Result<ContractFileResponse, String> {
    let conn = &mut establish_connection();

    let file_size = file_data.len() as i64;
    let stored_filename = format!("{}_{}.bin", contract_id, Uuid::new_v4());
    
    let file_path = get_files_dir().join(&stored_filename);
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    file.write_all(&file_data)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    let dto = ContractFileDTO {
        contract_fk: contract_id,
        file_name: stored_filename,
        orig_name: original_name,
        size_bytes: file_size,
        mime_type_txt: mime_type,
        file_type: ftype,
    };

    let result = diesel::insert_into(contract_files)
        .values(&dto)
        .get_result::<ContractFile>(conn)
        .map_err(|e| format!("Failed to save file record: {}", e))?;

    Ok(result.into())
}

pub async fn get_files_by_contract(contract_id: i32, ftype: &str) -> Result<Vec<ContractFileResponse>, String> {
    let conn = &mut establish_connection();

    let files = contract_files
        .filter(contract_fk.eq(contract_id))
        .filter(file_type.eq(ftype))
        .order(created_at.desc())
        .load::<ContractFile>(conn)
        .map_err(|e| format!("Failed to load files: {}", e))?;

    Ok(files.into_iter().map(|f| f.into()).collect())
}

pub async fn get_file_by_id(file_id: i32) -> Result<ContractFile, String> {
    let conn = &mut establish_connection();

    contract_files
        .filter(id.eq(file_id))
        .first::<ContractFile>(conn)
        .map_err(|e| format!("File not found: {}", e))
}

pub async fn delete_file(file_id: i32) -> Result<(), String> {
    let conn = &mut establish_connection();

    let file = contract_files
        .filter(id.eq(file_id))
        .first::<ContractFile>(conn)
        .map_err(|e| format!("File not found: {}", e))?;

    let file_path = get_files_dir().join(&file.file_name);
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    diesel::delete(contract_files.filter(id.eq(file_id)))
        .execute(conn)
        .map_err(|e| format!("Failed to delete file record: {}", e))?;

    Ok(())
}
