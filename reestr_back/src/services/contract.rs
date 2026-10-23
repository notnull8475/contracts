use crate::models::contract_history_models::ContractHistoryDTO;
use crate::models::contract_models::{
    Contract, ContractDTO, ContractListDTO, ContractListParams, ContractStatsResponse,
    ContractSummary, PaginatedContractsResponse,
};
use crate::schema::contract;
use crate::services::contract_history;
use crate::utils::db::establish_connection;
use crate::utils::utils::{require_non_empty, require_non_negative};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Integer, Nullable, Numeric, Text, Timestamptz};
use diesel::{QueryDsl, RunQueryDsl};
use std::collections::HashMap;

/// Общие проверки для добавления и обновления договора.
///
/// Форма проверяет то же самое, но API доступен любому клиенту с токеном —
/// без проверки на сервере в реестр попадали договоры с пустым номером,
/// датой окончания раньше начала и отрицательной ценой.
fn validate_contract(
    number: &str,
    date_from: Option<NaiveDateTime>,
    date_to: Option<NaiveDateTime>,
    contract_period: Option<i32>,
    price: Option<&bigdecimal::BigDecimal>,
) -> Result<(), String> {
    require_non_empty(number, "Номер договора")?;
    if let (Some(from), Some(to)) = (date_from, date_to) {
        if to < from {
            return Err("Дата окончания договора раньше даты начала".to_string());
        }
    }
    if let Some(period) = contract_period {
        if period < 0 {
            return Err("Срок действия договора не может быть отрицательным".to_string());
        }
    }
    require_non_negative(price, "Цена договора")
}

pub async fn add_contract(mut contract: ContractDTO) -> Result<Contract, String> {
    validate_contract(
        &contract.number,
        contract.date_from,
        contract.date_to,
        contract.contract_period,
        contract.price.as_ref(),
    )?;
    contract.number = contract.number.trim().to_string();

    let connection = &mut establish_connection();
    diesel::insert_into(contract::table)
        .values(contract)
        .get_result(connection)
        .map_err(|e| format!("Error to add contract: {}", e))
}

pub async fn remove_contract(contract_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();

    // Записи contract_files удалятся каскадом, а файлы на диске — нет:
    // запоминаем их имена до удаления договора.
    let stored_files: Vec<String> = crate::schema::contract_files::table
        .filter(crate::schema::contract_files::contract_fk.eq(contract_id))
        .select(crate::schema::contract_files::file_name)
        .load(connection)
        .map_err(|e| format!("Error to load contract files: {}", e))?;

    let result = diesel::delete(contract::table.filter(contract::id.eq(contract_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete contract: {}", e))?;

    crate::services::contract_files::remove_stored_files(&stored_files);
    Ok(result)
}

pub async fn update_contract(mut contract: Contract) -> Result<Contract, String> {
    validate_contract(
        &contract.number,
        contract.date_from,
        contract.date_to,
        contract.contract_period,
        contract.price.as_ref(),
    )?;
    contract.number = contract.number.trim().to_string();

    let connection = &mut establish_connection();

    let old: Contract = contract::table
        .filter(contract::id.eq(contract.id))
        .first(connection)
        .map_err(|e| format!("Contract not found: {}", e))?;

    // Changeset пишет null для None, а этих полей в форме нет —
    // без подстановки дата создания стёрлась бы при каждом сохранении.
    contract.created_time = old.created_time;
    contract.updated_time = Some(chrono::Utc::now().naive_utc());

    let result = diesel::update(contract::table)
        .filter(contract::id.eq(contract.id))
        .set(&contract)
        .get_result(connection)
        .map_err(|e| format!("Error to update contract: {}", e))?;

    if old.contract_status_id != contract.contract_status_id {
        // В истории пишем названия статусов, а не идентификаторы — их читает человек.
        let old_status = status_label(connection, old.contract_status_id);
        let new_status = status_label(connection, contract.contract_status_id);
        contract_history::record(
            connection,
            &ContractHistoryDTO {
                contract_id: contract.id,
                action: "status_changed".to_string(),
                old_value: Some(old_status.clone()),
                new_value: Some(new_status.clone()),
                description: Some(format!("{} → {}", old_status, new_status)),
            },
        );
    }

    Ok(result)
}

/// Читаемое название статуса для записи в историю ("не задан", если статуса нет).
fn status_label(conn: &mut diesel::PgConnection, status_id: Option<i32>) -> String {
    use crate::schema::dict_contract_status;

    let Some(sid) = status_id else {
        return "не задан".to_string();
    };

    dict_contract_status::table
        .filter(dict_contract_status::id.eq(sid))
        .select(dict_contract_status::name)
        .first::<String>(conn)
        .unwrap_or_else(|_| format!("#{}", sid))
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

// =============================================================================
// Пагинированный список договоров с фильтрами
// =============================================================================

#[derive(QueryableByName)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    total: i64,
}

#[derive(QueryableByName, Serialize)]
struct ContractRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    number: String,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    date_from: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    date_to: Option<NaiveDateTime>,
    #[diesel(sql_type = Nullable<Integer>)]
    contract_period: Option<i32>,
    #[diesel(sql_type = Integer)]
    organization_id: i32,
    #[diesel(sql_type = Nullable<Integer>)]
    type_of_validity: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    responsible_person_id: Option<i32>,
    #[diesel(sql_type = Nullable<Numeric>)]
    price: Option<bigdecimal::BigDecimal>,
    #[diesel(sql_type = Nullable<Integer>)]
    contract_status_id: Option<i32>,
}

use chrono::NaiveDateTime;
use serde::Serialize;

pub async fn list_contract_paginated(params: ContractListParams) -> Result<PaginatedContractsResponse, String> {
    let conn = &mut establish_connection();

    let page = params.page.unwrap_or(1).max(1);
    // per_page = 0 означает "все" (без LIMIT)
    let per_page_raw = params.per_page.unwrap_or(50);
    let show_all = per_page_raw == 0;
    let per_page = if show_all { 0i64 } else { per_page_raw.clamp(10, 10000) };
    let offset = (page - 1) * per_page;

    // Строим WHERE clause. Значение поиска передаётся bind-параметром ($1),
    // числовые фильтры валидируются и подставляются как целые числа.
    let mut conditions: Vec<String> = Vec::new();

    let search_pattern: Option<String> = params
        .search
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Экранируем спецсимволы LIKE, чтобы они искались буквально
            // (в PostgreSQL escape-символ LIKE по умолчанию — обратный слеш).
            let escaped = s
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_");
            format!("%{}%", escaped)
        });

    if search_pattern.is_some() {
        conditions.push(
            "(number ILIKE $1 \
              OR COALESCE(address, '') ILIKE $1 \
              OR COALESCE(comment, '') ILIKE $1 \
              OR COALESCE(additional_agreement, '') ILIKE $1 \
              OR organization_id IN (SELECT id FROM organization \
                    WHERE COALESCE(short_name_with_opf, '') ILIKE $1 \
                       OR COALESCE(full_name_with_opf, '') ILIKE $1) \
              OR responsible_person_id IN (SELECT id FROM responsible_person \
                    WHERE COALESCE(lastname, '') ILIKE $1))"
                .to_string(),
        );
    }

    if let Some(ref year) = params.year {
        if year != "all" {
            if let Ok(y) = year.parse::<i32>() {
                if (2000..=2100).contains(&y) {
                    conditions.push(format!("EXTRACT(YEAR FROM date_from)::integer = {}", y));
                }
            }
        }
    }

    if let Some(status) = params.status {
        if status > 0 {
            conditions.push(format!("contract_status_id = {}", status));
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Sort — только из whitelist. Ссылочные колонки сортируем по названию,
    // а не по id: в таблице видны название организации, статуса, типа и
    // фамилия ответственного, и пользователь ждёт алфавитный порядок.
    let sort_col = match params.sort_by.as_deref() {
        Some("date_from") => "date_from",
        Some("date_to") => "date_to",
        Some("number") => "number",
        Some("price") => "price",
        Some("contract_period") => "contract_period",
        Some("id") => "id",
        Some("contract_status_id") => {
            "(SELECT s.name FROM dict_contract_status s WHERE s.id = contract.contract_status_id)"
        }
        Some("organization_id") => {
            "(SELECT o.short_name_with_opf FROM organization o WHERE o.id = contract.organization_id)"
        }
        Some("type_of_validity") => {
            "(SELECT t.name FROM dict_type_of_validity t WHERE t.id = contract.type_of_validity)"
        }
        Some("responsible_person_id") => {
            "(SELECT p.lastname FROM responsible_person p WHERE p.id = contract.responsible_person_id)"
        }
        _ => "date_from",
    };
    let sort_order = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Считаем total
    let count_sql = format!("SELECT COUNT(*) as total FROM contract {}", where_clause);
    let count_query = sql_query(&count_sql);
    let total: i64 = match search_pattern {
        Some(ref pattern) => count_query
            .bind::<Text, _>(pattern.clone())
            .get_result::<CountResult>(conn),
        None => count_query.get_result::<CountResult>(conn),
    }
    .map(|r| r.total)
    .map_err(|e| format!("Error counting contracts: {}", e))?;

    // Получаем данные
    let limit_clause = if show_all {
        String::new()
    } else {
        format!("LIMIT {} OFFSET {}", per_page, offset)
    };

    let data_sql = format!(
        "SELECT id, number, date_from, date_to, contract_period, \
                organization_id, type_of_validity, responsible_person_id, \
                price, contract_status_id \
         FROM contract {} \
         ORDER BY {} {} NULLS LAST, id {} \
         {}",
        where_clause, sort_col, sort_order, sort_order, limit_clause
    );

    let data_query = sql_query(&data_sql);
    let rows: Vec<ContractRow> = match search_pattern {
        Some(ref pattern) => data_query
            .bind::<Text, _>(pattern.clone())
            .get_results(conn),
        None => data_query.get_results(conn),
    }
    .map_err(|e| format!("Error listing contracts: {}", e))?;

    let items: Vec<ContractListDTO> = rows
        .into_iter()
        .map(|r| ContractListDTO {
            id: r.id,
            number: r.number,
            date_from: r.date_from,
            date_to: r.date_to,
            contract_period: r.contract_period,
            organization_id: r.organization_id,
            type_of_validity: r.type_of_validity,
            responsible_person_id: r.responsible_person_id,
            price: r.price,
            contract_status_id: r.contract_status_id,
        })
        .collect();

    Ok(PaginatedContractsResponse {
        items,
        total,
        page,
        per_page,
    })
}

// =============================================================================
// Batch счётчики: файлы + доп соглашения для всех договоров
// =============================================================================

#[derive(QueryableByName)]
struct IdCountRow {
    #[diesel(sql_type = Integer)]
    contract_id: i32,
    #[diesel(sql_type = BigInt)]
    cnt: i64,
}

pub async fn batch_stats() -> Result<ContractStatsResponse, String> {
    let conn = &mut establish_connection();

    // Файлы (тип contract): GROUP BY contract_fk
    let file_rows: Vec<IdCountRow> = sql_query(
        "SELECT contract_fk as contract_id, COUNT(*) as cnt FROM contract_files WHERE file_type = 'contract' GROUP BY contract_fk"
    )
    .get_results(conn)
    .unwrap_or_default();

    // УПД (тип upd): GROUP BY contract_fk
    let upd_rows: Vec<IdCountRow> = sql_query(
        "SELECT contract_fk as contract_id, COUNT(*) as cnt FROM contract_files WHERE file_type = 'upd' GROUP BY contract_fk"
    )
    .get_results(conn)
    .unwrap_or_default();

    // Доп соглашения: GROUP BY contract_id
    let sa_rows: Vec<IdCountRow> = sql_query(
        "SELECT contract_id as contract_id, COUNT(*) as cnt FROM supplementary_agreement GROUP BY contract_id"
    )
    .get_results(conn)
    .unwrap_or_default();

    let mut files_map = HashMap::new();
    for row in file_rows {
        files_map.insert(row.contract_id, row.cnt);
    }

    let mut upd_map = HashMap::new();
    for row in upd_rows {
        upd_map.insert(row.contract_id, row.cnt);
    }

    let mut sa_map = HashMap::new();
    for row in sa_rows {
        sa_map.insert(row.contract_id, row.cnt);
    }

    Ok(ContractStatsResponse {
        files: files_map,
        upd: upd_map,
        supplementary: sa_map,
    })
}

// =============================================================================
// Сводка для главной страницы
// =============================================================================

#[derive(QueryableByName)]
struct SummaryRow {
    #[diesel(sql_type = BigInt)]
    contracts: i64,
    #[diesel(sql_type = BigInt)]
    active_contracts: i64,
    #[diesel(sql_type = BigInt)]
    organizations: i64,
    #[diesel(sql_type = BigInt)]
    responsible_persons: i64,
}

/// Счётчики для главной страницы одним запросом.
///
/// Раньше главная тянула полный список договоров и организаций (больше мегабайта)
/// только ради `length`, а «актуальные» считала по несуществующему полю и всегда
/// показывала 0. Актуальный договор — не расторгнутый и с датой окончания
/// в будущем либо без неё.
pub async fn summary() -> Result<ContractSummary, String> {
    let conn = &mut establish_connection();
    let row: SummaryRow = sql_query(
        "SELECT \
            (SELECT COUNT(*) FROM contract) AS contracts, \
            (SELECT COUNT(*) FROM contract c \
                WHERE (c.date_to IS NULL OR c.date_to >= CURRENT_DATE) \
                  AND NOT EXISTS (SELECT 1 FROM dict_contract_status s \
                                  WHERE s.id = c.contract_status_id \
                                    AND s.name ILIKE 'расторгнут%')) AS active_contracts, \
            (SELECT COUNT(*) FROM organization) AS organizations, \
            (SELECT COUNT(*) FROM responsible_person) AS responsible_persons",
    )
    .get_result(conn)
    .map_err(|e| format!("Error building summary: {}", e))?;

    Ok(ContractSummary {
        contracts: row.contracts,
        active_contracts: row.active_contracts,
        organizations: row.organizations,
        responsible_persons: row.responsible_persons,
    })
}
