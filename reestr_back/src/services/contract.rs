use crate::models::contract_models::{Contract, ContractDTO, ContractListDTO, ContractListParams, ContractStatsResponse, PaginatedContractsResponse};
use crate::schema::contract;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Integer, Nullable, Text, Timestamp, Timestamptz, Numeric};
use diesel::{QueryDsl, RunQueryDsl};
use std::collections::HashMap;

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
    #[diesel(sql_type = Nullable<Timestamp>)]
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

    // Строим WHERE clause — прямая интерполяция (без $N placeholders)
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref search) = params.search {
        if !search.is_empty() {
            // Экранируем одинарные кавычки для SQL-безопасности
            let safe_search = search.replace('\'', "''");
            conditions.push(format!("number ILIKE '%{}%'", safe_search));
        }
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

    // Sort — только из whitelist
    let sort_col = match params.sort_by.as_deref() {
        Some("date_from") => "date_from",
        Some("date_to") => "date_to",
        Some("number") => "number",
        Some("price") => "price",
        Some("contract_status_id") => "contract_status_id",
        Some("organization_id") => "organization_id",
        _ => "id",
    };
    let sort_order = match params.sort_order.as_deref() {
        Some("desc") => "DESC",
        _ => "ASC",
    };

    // Считаем total
    let count_sql = format!("SELECT COUNT(*) as total FROM contract {}", where_clause);
    let total: i64 = sql_query(&count_sql)
        .get_result::<CountResult>(conn)
        .map(|r| r.total)
        .unwrap_or(0);

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
         ORDER BY {} {} \
         {}",
        where_clause, sort_col, sort_order, limit_clause
    );

    let rows: Vec<ContractRow> = sql_query(&data_sql)
        .get_results(conn)
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
