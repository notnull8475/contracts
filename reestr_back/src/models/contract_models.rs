use crate::schema::contract;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
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
    pub actual: Option<bool>,
    pub file_link: Option<String>,
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
    pub actual: Option<bool>,
    pub file_link: Option<String>,
}

/// DTO только для импорта — включает все поля без id/timestamps
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct ContractImportDTO {
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
    pub actual: Option<bool>,
    pub file_link: Option<String>,
}
