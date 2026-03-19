use crate::models::contract_status_models::{
    ContractStatus, ContractStatusInsertDTO, ContractStatusUpdateDTO,
};
use crate::schema::dict_contract_status::dsl::*;
use crate::utils::db::establish_connection;
use diesel::prelude::*;

pub async fn list_statuses() -> Result<Vec<ContractStatus>, String> {
    let conn = &mut establish_connection();
    dict_contract_status
        .order(id.asc())
        .load::<ContractStatus>(conn)
        .map_err(|e| format!("Ошибка получения статусов: {}", e))
}

pub async fn add_status(dto: ContractStatusInsertDTO) -> Result<ContractStatus, String> {
    let conn = &mut establish_connection();
    diesel::insert_into(dict_contract_status)
        .values(&dto)
        .get_result::<ContractStatus>(conn)
        .map_err(|e| format!("Ошибка добавления статуса: {}", e))
}

pub async fn update_status(dto: ContractStatusUpdateDTO) -> Result<ContractStatus, String> {
    let conn = &mut establish_connection();
    diesel::update(dict_contract_status.filter(id.eq(dto.id)))
        .set(&dto)
        .get_result::<ContractStatus>(conn)
        .map_err(|e| format!("Ошибка обновления статуса: {}", e))
}

pub async fn remove_status(status_id: i32) -> Result<usize, String> {
    let conn = &mut establish_connection();
    // FK ON DELETE SET NULL — обнуляет contract_status_id в договорах автоматически
    diesel::delete(dict_contract_status.filter(id.eq(status_id)))
        .execute(conn)
        .map_err(|e| format!("Ошибка удаления статуса: {}", e))
}
