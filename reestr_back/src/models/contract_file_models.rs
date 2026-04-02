use crate::schema::contract_files;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract_files)]
pub struct ContractFile {
    pub id: i32,
    pub contract_fk: i32,
    pub file_name: String,
    pub orig_name: String,
    pub size_bytes: i64,
    pub mime_type_txt: String,
    pub created_at: Option<DateTime<Utc>>,
    pub file_type: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract_files)]
pub struct ContractFileDTO {
    pub contract_fk: i32,
    pub file_name: String,
    pub orig_name: String,
    pub size_bytes: i64,
    pub mime_type_txt: String,
    pub file_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractFileResponse {
    pub id: i32,
    pub contract_fk: i32,
    pub original_name: String,
    pub filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub file_type: String,
}

impl From<ContractFile> for ContractFileResponse {
    fn from(file: ContractFile) -> Self {
        ContractFileResponse {
            id: file.id,
            contract_fk: file.contract_fk,
            original_name: file.orig_name,
            filename: file.file_name,
            file_size: file.size_bytes,
            mime_type: file.mime_type_txt,
            created_at: file.created_at,
            file_type: file.file_type,
        }
    }
}
