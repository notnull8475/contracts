use crate::models::supplementary_agreement_models::{
    SupplementaryAgreement, SupplementaryAgreementDTO, SupplementaryAgreementUpdateDTO,
};
use crate::schema::supplementary_agreement::dsl::*;
use crate::utils::db::establish_connection;
use diesel::prelude::*;

pub async fn list_by_contract(cid: i32) -> Result<Vec<SupplementaryAgreement>, String> {
    let conn = &mut establish_connection();
    supplementary_agreement
        .filter(contract_id.eq(cid))
        .order(created_at.desc())
        .load::<SupplementaryAgreement>(conn)
        .map_err(|e| format!("Ошибка получения доп соглашений: {}", e))
}

pub async fn add(dto: SupplementaryAgreementDTO) -> Result<SupplementaryAgreement, String> {
    let conn = &mut establish_connection();
    diesel::insert_into(supplementary_agreement)
        .values(&dto)
        .get_result::<SupplementaryAgreement>(conn)
        .map_err(|e| format!("Ошибка добавления доп соглашения: {}", e))
}

pub async fn update(dto: SupplementaryAgreementUpdateDTO) -> Result<SupplementaryAgreement, String> {
    let conn = &mut establish_connection();
    diesel::update(supplementary_agreement.filter(id.eq(dto.id)))
        .set(&dto)
        .get_result::<SupplementaryAgreement>(conn)
        .map_err(|e| format!("Ошибка обновления доп соглашения: {}", e))
}

pub async fn remove(sid: i32) -> Result<usize, String> {
    let conn = &mut establish_connection();
    diesel::delete(supplementary_agreement.filter(id.eq(sid)))
        .execute(conn)
        .map_err(|e| format!("Ошибка удаления доп соглашения: {}", e))
}

pub async fn count_by_contract(cid: i32) -> Result<i64, String> {
    let conn = &mut establish_connection();
    supplementary_agreement
        .filter(contract_id.eq(cid))
        .count()
        .get_result(conn)
        .map_err(|e| format!("Ошибка подсчёта доп соглашений: {}", e))
}
