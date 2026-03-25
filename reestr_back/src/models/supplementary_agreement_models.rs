use crate::schema::supplementary_agreement;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = supplementary_agreement)]
pub struct SupplementaryAgreement {
    pub id: i32,
    pub contract_id: i32,
    pub number: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub file_link: Option<String>,
    pub price: Option<BigDecimal>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = supplementary_agreement)]
pub struct SupplementaryAgreementDTO {
    pub contract_id: i32,
    pub number: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub file_link: Option<String>,
    pub price: Option<BigDecimal>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = supplementary_agreement)]
pub struct SupplementaryAgreementUpdateDTO {
    pub id: i32,
    pub number: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub file_link: Option<String>,
    pub price: Option<BigDecimal>,
}
