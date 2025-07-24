use crate::models::models::{Organization};
use crate::schema::organization;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_responsible_person(
    person: Organization,
) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(organization::table)
        .values(person)
        .get_result(connection)
        .map_err(|e| format!("Error to add responsible person: {}", e.to_string()));
    result
}
pub async fn remove_organization(person_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(organization::table.filter(organization::id.eq(person_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete responsible person: {}", e))
}

pub async fn update_organization(
    person: Organization,
) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    diesel::update(organization::table)
        .filter(organization::id.eq(person.id))
        .set(person)
        .get_result(connection)
        .map_err(|e| format!("Error to update responsible person: {}", e))
}

pub async fn list_organization() -> Result<Vec<Organization>, String> {
    let connection = &mut establish_connection();
    organization::table
        .select(organization::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list responsible person: {}", e))
}
pub async fn get_organization(person_id: i32) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    organization::table
        .filter(organization::id.eq(person_id))
        .select(organization::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get responsible person: {}", e))
}
