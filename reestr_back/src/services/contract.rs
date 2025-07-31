use crate::models::models::{Contract, ContractDTO};
use crate::schema::contract;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_contract(contract: ContractDTO) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(contract::table)
        .values(contract)
        .get_result(connection)
        .map_err(|e| format!("Error to add contract: {}", e.to_string()));
    result
}

pub async fn remove_contract(contract_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(contract::table.filter(contract::id.eq(contract_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete contract: {}", e))
}

pub async fn update_contract(contract: Contract) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    diesel::update(contract::table)
        .filter(contract::id.eq(contract.id))
        .set(contract)
        .get_result(connection)
        .map_err(|e| format!("Error to update contract: {}", e))
}

pub async fn list_contract() -> Result<Vec<Contract>, String> {
    let connection = &mut establish_connection();
    contract::table
        .select(contract::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list contract: {}", e))
}
pub async fn get_contract(contract_id: i32) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    contract::table
        .filter(contract::id.eq(contract_id))
        .select(contract::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get contract: {}", e))
}
