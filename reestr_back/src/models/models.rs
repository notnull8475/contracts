use crate::schema::{dict_type_of_validity, responsible_person};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

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

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = responsible_person)]
pub struct ResponsiblePerson {
    pub id: i32,
    pub firstname: String,
    pub name: String,
    pub lastname: Option<String>,
    pub user_id: Option<i32>,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = responsible_person)]
pub struct ResponsiblePersonDTO {
    pub firstname: String,
    pub name: String,
    pub lastname: Option<String>,
    pub user_id: Option<i32>,
}
