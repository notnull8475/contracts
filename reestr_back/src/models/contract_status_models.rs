use crate::schema::dict_contract_status;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = dict_contract_status)]
pub struct ContractStatus {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = dict_contract_status)]
pub struct ContractStatusInsertDTO {
    pub name: String,
    pub color: String,
}

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = dict_contract_status)]
pub struct ContractStatusUpdateDTO {
    pub id: i32,
    pub name: String,
    pub color: String,
}
