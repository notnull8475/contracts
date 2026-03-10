use crate::schema::organization;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = organization)]
pub struct Organization {
    pub id: i32,
    pub short_name_with_opf: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub legal_address: Option<String>,
    pub management_post: Option<String>,
    pub management_name: Option<String>,
    pub ogrn: Option<String>,
    pub full_name_with_opf: Option<String>,
    pub opf_full: Option<String>,
    pub opf_short: Option<String>,
}
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = organization)]
pub struct NewOrganizationDTO {
    pub short_name_with_opf: String,
    pub inn: i64,
    pub fact_address: Option<String>,
    pub legal_address: Option<String>,
    pub management_post: Option<String>,
    pub management_name: Option<String>,
    pub ogrn: Option<String>,
    pub full_name_with_opf: Option<String>,
    pub opf_full: Option<String>,
    pub opf_short: Option<String>,
}
