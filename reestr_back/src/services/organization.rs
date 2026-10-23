use crate::models::organization_models::{NewOrganizationDTO, Organization};
use crate::schema::organization;
use crate::utils::db::establish_connection;
use crate::utils::utils::require_non_empty;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

/// ИНН юрлица — 10 цифр, ИП — 12. Форма это проверяет, API — тоже должен.
pub fn validate_organization(name: &str, inn: i64) -> Result<(), String> {
    require_non_empty(name, "Название организации")?;
    let digits = inn.to_string().len();
    if inn <= 0 || !(digits == 10 || digits == 12) {
        return Err("ИНН должен состоять из 10 или 12 цифр".to_string());
    }
    Ok(())
}

pub async fn add_organization(mut organization: NewOrganizationDTO) -> Result<Organization, String> {
    validate_organization(&organization.short_name_with_opf, organization.inn)?;
    organization.short_name_with_opf = organization.short_name_with_opf.trim().to_string();

    let connection = &mut establish_connection();
    diesel::insert_into(organization::table)
        .values(organization)
        .get_result(connection)
        .map_err(|e| format!("Error to add organization: {}", e))
}
pub async fn remove_organization(organization_id: i32) -> Result<usize, String> {
    let connection = &mut establish_connection();
    diesel::delete(organization::table.filter(organization::id.eq(organization_id)))
        .execute(connection)
        .map_err(|e| format!("Error to delete organization: {}", e))
}

pub async fn update_organization(mut organization: Organization) -> Result<Organization, String> {
    validate_organization(&organization.short_name_with_opf, organization.inn)?;
    organization.short_name_with_opf = organization.short_name_with_opf.trim().to_string();

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

#[cfg(test)]
mod tests {
    use super::validate_organization;

    #[test]
    fn инн_10_и_12_цифр_проходит_остальное_нет() {
        assert!(validate_organization("ООО Ромашка", 1655000000).is_ok());
        assert!(validate_organization("ИП Иванов", 165500000012).is_ok());
        assert!(validate_organization("ООО Ромашка", 0).is_err());
        assert!(validate_organization("ООО Ромашка", 12345).is_err());
        assert!(validate_organization("   ", 1655000000).is_err());
    }
}
