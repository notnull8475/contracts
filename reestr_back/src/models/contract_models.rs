use crate::schema::contract;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, QueryableByName};
use diesel::sql_types::{Integer, Nullable, Text, Timestamp, Timestamptz, Numeric};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct Contract {
    pub id: i32,
    pub number: String,
    pub date_from: Option<NaiveDateTime>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub address: Option<String>,
    pub additional_agreement: Option<String>,
    pub comment: Option<String>,
    pub date_to: Option<NaiveDateTime>,
    pub contract_period: Option<i32>,
    pub created_time: Option<NaiveDateTime>,
    pub updated_time: Option<NaiveDateTime>,
    pub file_link: Option<String>,
    pub contract_status_id: Option<i32>,
    pub price: Option<BigDecimal>,
    pub pricelist_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct ContractDTO {
    pub number: String,
    pub date_from: Option<NaiveDateTime>,
    pub contract_period: Option<i32>,
    pub date_to: Option<NaiveDateTime>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub address: Option<String>,
    pub additional_agreement: Option<String>,
    pub comment: Option<String>,
    pub file_link: Option<String>,
    pub contract_status_id: Option<i32>,
    pub price: Option<BigDecimal>,
    pub pricelist_id: Option<i32>,
}

/// Лёгкий DTO для отображения в таблице (без comment, file_link, additional_agreement)
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractListDTO {
    pub id: i32,
    pub number: String,
    pub date_from: Option<NaiveDateTime>,
    pub date_to: Option<NaiveDateTime>,
    pub contract_period: Option<i32>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub price: Option<BigDecimal>,
    pub contract_status_id: Option<i32>,
}

/// Ответ пагинированного списка
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedContractsResponse {
    pub items: Vec<ContractListDTO>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Счётчики файлов и доп соглашений (batch)
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractStatsResponse {
    pub files: std::collections::HashMap<i32, i64>,
    pub supplementary: std::collections::HashMap<i32, i64>,
}

/// Параметры фильтрации для списка договоров
#[derive(Deserialize, Debug)]
pub struct ContractListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub year: Option<String>,
    pub status: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
