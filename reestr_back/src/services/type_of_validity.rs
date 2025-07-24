use crate::models::models::TypeOfValidity;
use crate::schema::dict_type_of_validity;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_type(adding_name: String) -> Result<TypeOfValidity, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(dict_type_of_validity::table)
        .values(dict_type_of_validity::name.eq(adding_name))
        .get_result(connection)
        .map_err(|e| format!("Error to add type: {}", e.to_string()));
    result
}
pub async fn remove_type(type_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(dict_type_of_validity::table.filter(dict_type_of_validity::id.eq(type_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete type: {}", e))
}

pub async fn list_type() -> Result<Vec<TypeOfValidity>, String> {
    let connection = &mut establish_connection();
    dict_type_of_validity::table
        .select(dict_type_of_validity::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list type: {}", e))
}
pub async fn get_type(id: i32) -> Result<TypeOfValidity, String> {
    let connection = &mut establish_connection();
    dict_type_of_validity::table
        .filter(dict_type_of_validity::id.eq(id))
        .select(dict_type_of_validity::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get type: {}", e))
}
