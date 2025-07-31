use crate::models::organization_models::{Organization, OrganizationDTO};
use crate::schema::organization;
use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn add_organization(organization: OrganizationDTO) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(organization::table)
        .values(organization)
        .get_result(connection)
        .map_err(|e| format!("Error to add organization: {}", e.to_string()));
    result
}
pub async fn remove_organization(organization_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(organization::table.filter(organization::id.eq(organization_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete organization: {}", e))
}

pub async fn update_organization(organization: Organization) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    diesel::update(organization::table)
        .filter(organization::id.eq(organization.id))
        .set(organization)
        .get_result(connection)
        .map_err(|e| format!("Error to update organization: {}", e))
}

pub async fn list_organization() -> Result<Vec<Organization>, String> {
    let connection = &mut establish_connection();
    organization::table
        .select(organization::all_columns)
        .get_results(connection)
        .map_err(|e| format!("Error to list organization: {}", e))
}
pub async fn get_organization(organization_id: i32) -> Result<Organization, String> {
    let connection = &mut establish_connection();
    organization::table
        .filter(organization::id.eq(organization_id))
        .select(organization::all_columns)
        .first(connection)
        .map_err(|e| format!("Error to get organization: {}", e))
}
