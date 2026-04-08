use crate::models::contract_history_models::ContractHistoryDTO;
use crate::models::supplementary_agreement_models::{
    SupplementaryAgreement, SupplementaryAgreementDTO, SupplementaryAgreementUpdateDTO,
};
use crate::schema::contract::dsl as contract_dsl;
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

    let result = diesel::insert_into(supplementary_agreement)
        .values(&dto)
        .get_result::<SupplementaryAgreement>(conn)
        .map_err(|e| format!("Ошибка добавления доп соглашения: {}", e))?;

    let sa_num = dto.number.as_deref().unwrap_or("б/н");
    let sa_date = dto.date_from
        .map(|d| d.format("%d.%m.%Y").to_string())
        .unwrap_or_else(|| "без даты".to_string());

    let _ = diesel::insert_into(crate::schema::contract_history::table)
        .values(&ContractHistoryDTO {
            contract_id: dto.contract_id,
            action: "sa_added".to_string(),
            old_value: None,
            new_value: Some(format!("№{} от {}", sa_num, sa_date)),
            description: dto.description.clone(),
        })
        .execute(conn);

    if let Some(ref new_price) = dto.price {
        let price_str = new_price.to_string();
        let _ = diesel::update(contract_dsl::contract.filter(contract_dsl::id.eq(dto.contract_id)))
            .set(contract_dsl::price.eq(new_price))
            .execute(conn);

        let _ = diesel::insert_into(crate::schema::contract_history::table)
            .values(&ContractHistoryDTO {
                contract_id: dto.contract_id,
                action: "price_changed".to_string(),
                old_value: None,
                new_value: Some(price_str),
                description: Some("Цена изменена из доп. соглашения".to_string()),
            })
            .execute(conn);
    }

    Ok(result)
}

pub async fn update(dto: SupplementaryAgreementUpdateDTO) -> Result<SupplementaryAgreement, String> {
    let conn = &mut establish_connection();

    let old: SupplementaryAgreement = supplementary_agreement
        .filter(id.eq(dto.id))
        .first(conn)
        .map_err(|e| format!("Доп соглашение не найдено: {}", e))?;

    let result = diesel::update(supplementary_agreement.filter(id.eq(dto.id)))
        .set(&dto)
        .get_result::<SupplementaryAgreement>(conn)
        .map_err(|e| format!("Ошибка обновления доп соглашения: {}", e))?;

    let sa_num = dto.number.as_deref().unwrap_or("б/н");
    let sa_date = dto.date_from
        .map(|d| d.format("%d.%m.%Y").to_string())
        .unwrap_or_else(|| "без даты".to_string());

    let mut changes: Vec<String> = Vec::new();
    if dto.number != old.number {
        changes.push(format!("номер: {} → {}", old.number.as_deref().unwrap_or("-"), sa_num));
    }
    if dto.date_from != old.date_from {
        changes.push("дата изменена".to_string());
    }
    if dto.description != old.description {
        changes.push("описание изменено".to_string());
    }
    if dto.price != old.price {
        let old_price = old.price.map(|p| p.to_string()).unwrap_or_else(|| "нет".to_string());
        let new_price_val = dto.price.clone().map(|p| p.to_string()).unwrap_or_else(|| "нет".to_string());
        changes.push(format!("цена: {} → {}", old_price, new_price_val));

        if let Some(ref np) = dto.price {
            let _ = diesel::update(contract_dsl::contract.filter(contract_dsl::id.eq(old.contract_id)))
                .set(contract_dsl::price.eq(np))
                .execute(conn);

            let _ = diesel::insert_into(crate::schema::contract_history::table)
                .values(&ContractHistoryDTO {
                    contract_id: old.contract_id,
                    action: "price_changed".to_string(),
                    old_value: Some(old_price),
                    new_value: Some(np.to_string()),
                    description: Some("Цена изменена из доп. соглашения".to_string()),
                })
                .execute(conn);
        }
    }

    if !changes.is_empty() {
        let _ = diesel::insert_into(crate::schema::contract_history::table)
            .values(&ContractHistoryDTO {
                contract_id: old.contract_id,
                action: "sa_updated".to_string(),
                old_value: None,
                new_value: Some(format!("№{} от {}", sa_num, sa_date)),
                description: Some(changes.join("; ")),
            })
            .execute(conn);
    }

    Ok(result)
}

pub async fn remove(sid: i32) -> Result<usize, String> {
    let conn = &mut establish_connection();

    let old: SupplementaryAgreement = supplementary_agreement
        .filter(id.eq(sid))
        .first(conn)
        .map_err(|e| format!("Доп соглашение не найдено: {}", e))?;

    let sa_info = format!(
        "№{} от {}",
        old.number.as_deref().unwrap_or("б/н"),
        old.date_from.map(|d| d.format("%d.%m.%Y").to_string()).unwrap_or_else(|| "без даты".to_string())
    );

    let result = diesel::delete(supplementary_agreement.filter(id.eq(sid)))
        .execute(conn)
        .map_err(|e| format!("Ошибка удаления доп соглашения: {}", e))?;

    let _ = diesel::insert_into(crate::schema::contract_history::table)
        .values(&ContractHistoryDTO {
            contract_id: old.contract_id,
            action: "sa_deleted".to_string(),
            old_value: Some(sa_info),
            new_value: None,
            description: old.description,
        })
        .execute(conn);

    Ok(result)
}

pub async fn count_by_contract(cid: i32) -> Result<i64, String> {
    let conn = &mut establish_connection();
    supplementary_agreement
        .filter(contract_id.eq(cid))
        .count()
        .get_result(conn)
        .map_err(|e| format!("Ошибка подсчёта доп соглашений: {}", e))
}
