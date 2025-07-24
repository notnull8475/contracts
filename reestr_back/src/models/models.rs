use crate::schema::{contract, dict_type_of_validity, organization, responsible_person};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use actix_web::delete;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct Contract {
    pub id: i32,
    pub number: String,
    pub date: Option<SystemTime>,
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
    pub date: Option<SystemTime>,
    pub organization_id: i32,
    pub type_of_validity: Option<i32>,
    pub responsible_person_id: Option<i32>,
    pub address: Option<String>,
    pub additional_agreement: Option<String>,
    pub comment: Option<String>,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = dict_type_of_validity)]
pub struct TypeOfValidity {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NewType {
    pub name: String,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub address: Option<String>,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct OrganizationDTO {
    pub name: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub address: Option<String>,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = responsible_person)]
pub struct ResponsiblePerson {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub name: Option<String>,
    pub user_id: Option<i32>,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = responsible_person)]
pub struct ResponsiblePersonDTO {
    pub firstname: String,
    pub lastname: String,
    pub name: Option<String>,
    pub user_id: Option<i32>,
}