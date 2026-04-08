use crate::models::contract_history_models::{ContractHistory, ContractHistoryDTO};
use crate::schema::contract_history::dsl::*;
use crate::utils::db::establish_connection;
use diesel::prelude::*;

pub async fn list_by_contract(cid: i32) -> Result<Vec<ContractHistory>, String> {
    let conn = &mut establish_connection();
    contract_history
        .filter(contract_id.eq(cid))
        .order(created_at.desc())
        .load::<ContractHistory>(conn)
        .map_err(|e| format!("Ошибка получения истории: {}", e))
}

pub async fn add_entry(dto: ContractHistoryDTO) -> Result<ContractHistory, String> {
    let conn = &mut establish_connection();
    diesel::insert_into(contract_history)
        .values(&dto)
        .get_result::<ContractHistory>(conn)
        .map_err(|e| format!("Ошибка записи в историю: {}", e))
}
