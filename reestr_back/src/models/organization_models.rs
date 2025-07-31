use crate::schema::organization;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub address: Option<String>,
}
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct OrganizationDTO {
    pub name: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewOrganizationDTO {
    pub name: String,
    pub inn: String,
    pub fact_address: Option<String>,
    pub address: Option<String>,
}
impl NewOrganizationDTO {
    pub fn validate_and_convert(&self) -> Result<OrganizationDTO, String> {
        // Преобразование ИНН
        let inn = i64::from_str(&self.inn).map_err(|_| "Invalid INN format".to_string())?;

        // Создание структуры Organization
        Ok(OrganizationDTO {
            name: self.name.clone(),
            inn,
            fact_address: self.fact_address.clone(),
            address: self.address.clone(),
        })
    }
}
