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

/// Пишет запись в историю договора на уже открытом соединении.
///
/// История вторична по отношению к основной операции: если запись не удалась,
/// операцию не откатываем, но ошибку логируем, а не глотаем молча.
pub fn record(conn: &mut PgConnection, dto: &ContractHistoryDTO) {
    if let Err(e) = diesel::insert_into(contract_history).values(dto).execute(conn) {
        log::error!(
            "Не удалось записать историю договора {} ({}): {}",
            dto.contract_id,
            dto.action,
            e
        );
    }
}
