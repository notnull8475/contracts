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
    sa_id: Option<i32>,
) -> Result<ContractFileResponse, String> {
    let conn = &mut establish_connection();

    let file_size = file_data.len() as i64;
    let stored_filename = format!("{}_{}.bin", contract_id, Uuid::new_v4());
    
    let file_path = get_files_dir().join(&stored_filename);
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    // При ошибке записи (например, ENOSPC) на диске остаётся обрезанный файл без записи в БД.
    if let Err(e) = file.write_all(&file_data).and_then(|_| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(&file_path);
        return Err(format!("Failed to write file: {}", e));
    }

    let dto = ContractFileDTO {
        contract_fk: contract_id,
        file_name: stored_filename,
        orig_name: original_name,
        size_bytes: file_size,
        mime_type_txt: mime_type,
        file_type: ftype,
        supplementary_agreement_id: sa_id,
    };

    let result = match diesel::insert_into(contract_files)
        .values(&dto)
        .get_result::<ContractFile>(conn)
    {
        Ok(row) => row,
        Err(e) => {
            // Запись в БД не удалась (например, договора не существует) —
            // убираем уже записанный файл, чтобы не копить «сироты» на диске.
            let _ = fs::remove_file(&file_path);
            return Err(format!("Failed to save file record: {}", e));
        }
    };

    Ok(result.into())
}

/// Проверяет, что файл привязывается к существующей паре «договор + соглашение»:
/// для `supplementary` соглашение обязательно и должно принадлежать договору,
/// для остальных типов соглашения быть не должно.
pub async fn check_upload_target(
    contract_id: i32,
    ftype: &str,
    sa_id: Option<i32>,
) -> Result<(), String> {
    use crate::schema::supplementary_agreement::dsl as sa;

    match (ftype, sa_id) {
        ("supplementary", None) => Err("Для файла доп. соглашения не указано соглашение".into()),
        ("supplementary", Some(sid)) => {
            let conn = &mut establish_connection();
            let owner: Option<i32> = sa::supplementary_agreement
                .filter(sa::id.eq(sid))
                .select(sa::contract_id)
                .first(conn)
                .optional()
                .map_err(|e| format!("Failed to check supplementary agreement: {}", e))?;
            match owner {
                Some(cid) if cid == contract_id => Ok(()),
                Some(_) => Err("Доп. соглашение относится к другому договору".into()),
                None => Err("Доп. соглашение не найдено".into()),
            }
        }
        (_, Some(_)) => Err("Соглашение указывается только для файлов доп. соглашений".into()),
        (_, None) => Ok(()),
    }
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

pub async fn get_files_by_sa(sa_id: i32) -> Result<Vec<ContractFileResponse>, String> {
    let conn = &mut establish_connection();

    let files = contract_files
        .filter(supplementary_agreement_id.eq(sa_id))
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

    // Сначала запись в БД: если её удаление упадёт, файл должен остаться на месте,
    // иначе запись будет указывать в пустоту и скачивание вернёт 500.
    diesel::delete(contract_files.filter(id.eq(file_id)))
        .execute(conn)
        .map_err(|e| format!("Failed to delete file record: {}", e))?;

    remove_stored_files(&[file.file_name]);

    Ok(())
}

/// Удаляет файлы из хранилища по именам из `contract_files.file_name`.
/// Ошибки только логируются: запись в БД уже удалена, откатывать нечего.
pub fn remove_stored_files(names: &[String]) {
    for name in names {
        let path = get_files_dir().join(name);
        if let Err(e) = fs::remove_file(&path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                log::error!("Failed to delete stored file {}: {}", path.display(), e);
            }
        }
    }
}
