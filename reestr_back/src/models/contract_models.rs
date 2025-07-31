use crate::schema::contract;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct Contract {
    pub id: i32,
    pub number: String,
    pub date: Option<NaiveDateTime>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub address: Option<String>,
    pub additional_agreement: Option<String>,
    pub comment: Option<String>,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct ContractDTO {
    pub number: String,
    pub date: Option<NaiveDateTime>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub address: Option<String>,
    pub additional_agreement: Option<String>,
    pub comment: Option<String>,
}
