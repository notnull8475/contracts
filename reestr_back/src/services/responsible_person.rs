use crate::models::models::{ResponsiblePerson, ResponsiblePersonDTO};
use crate::schema::responsible_person;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_responsible_person(
    person: ResponsiblePersonDTO,
) -> Result<ResponsiblePerson, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(responsible_person::table)
        .values(person)
        .get_result(connection)
        .map_err(|e| format!("Error to add responsible person: {}", e.to_string()));
    result
}
pub async fn remove_responsible_person(person_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(responsible_person::table.filter(responsible_person::id.eq(person_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete responsible person: {}", e))
}

pub async fn update_responsible_person(
    person: ResponsiblePerson,
) -> Result<ResponsiblePerson, String> {
    let connection = &mut establish_connection();
    diesel::update(responsible_person::table)
        .filter(responsible_person::id.eq(person.id))
        .set(person)
        .get_result(connection)
        .map_err(|e| format!("Error to update responsible person: {}", e))
}

pub async fn list_responsible_person() -> Result<Vec<ResponsiblePerson>, String> {
    let connection = &mut establish_connection();
    responsible_person::table
        .select(responsible_person::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list responsible person: {}", e))
}
pub async fn get_responsible_person(person_id: i32) -> Result<ResponsiblePerson, String> {
    let connection = &mut establish_connection();
    responsible_person::table
        .filter(responsible_person::id.eq(person_id))
        .select(responsible_person::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get responsible person: {}", e))
}
