use crate::models::pricelist_models::{PricelistInsertDTO, PricelistItem, PricelistUpdateDTO};
use crate::schema::dict_pricelist::dsl::*;
use crate::utils::db::establish_connection;
use diesel::prelude::*;

pub async fn list_pricelist() -> Result<Vec<PricelistItem>, String> {
    let conn = &mut establish_connection();
    dict_pricelist
        .order(id.asc())
        .load::<PricelistItem>(conn)
        .map_err(|e| format!("Ошибка получения прайса: {}", e))
}

pub async fn add_pricelist(dto: PricelistInsertDTO) -> Result<PricelistItem, String> {
    let conn = &mut establish_connection();
    diesel::insert_into(dict_pricelist)
        .values(&dto)
        .get_result::<PricelistItem>(conn)
        .map_err(|e| format!("Ошибка добавления позиции прайса: {}", e))
}

pub async fn update_pricelist(dto: PricelistUpdateDTO) -> Result<PricelistItem, String> {
    let conn = &mut establish_connection();
    diesel::update(dict_pricelist.filter(id.eq(dto.id)))
        .set(&dto)
        .get_result::<PricelistItem>(conn)
        .map_err(|e| format!("Ошибка обновления позиции прайса: {}", e))
}

pub async fn remove_pricelist(pid: i32) -> Result<usize, String> {
    let conn = &mut establish_connection();
    diesel::delete(dict_pricelist.filter(id.eq(pid)))
        .execute(conn)
        .map_err(|e| format!("Ошибка удаления позиции прайса: {}", e))
}
