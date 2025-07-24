use crate::schema::{contract, dict_type_of_validity, organization, responsible_person};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = contract)]
pub struct Contract {
    id: i32,
    number: String,
    date: SystemTime,
    organization_id: i32,
    type_of_validity: i32,
    responsible_person_id: i32,
    address: String,
    additional_agreement: String,
    comment: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = dict_type_of_validity)]
pub struct TypeOfValidity {
    id: i32,
    name: String,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct Organization {
    id: i32,
    name: String,
    inn: i64,
    fact_address: String,
    address: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = responsible_person)]
pub struct ResponsiblePerson {
    id: i32,
    firstname: String,
    lastname: String,
    name: String,
    user_id: i32,
}
