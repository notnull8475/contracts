use crate::models::models::{Contract};
use crate::schema::contract;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_responsible_person(
    person: Contract,
) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(contract::table)
        .values(person)
        .get_result(connection)
        .map_err(|e| format!("Error to add responsible person: {}", e.to_string()));
    result
}

pub async fn remove_contract(person_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(contract::table.filter(contract::id.eq(person_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete responsible person: {}", e))
}

pub async fn update_contract(
    person: Contract,
) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    diesel::update(contract::table)
        .filter(contract::id.eq(person.id))
        .set(person)
        .get_result(connection)
        .map_err(|e| format!("Error to update responsible person: {}", e))
}

pub async fn list_contract() -> Result<Vec<Contract>, String> {
    let connection = &mut establish_connection();
    contract::table
        .select(contract::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list responsible person: {}", e))
}
pub async fn get_contract(person_id: i32) -> Result<Contract, String> {
    let connection = &mut establish_connection();
    contract::table
        .filter(contract::id.eq(person_id))
        .select(contract::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get responsible person: {}", e))
}
