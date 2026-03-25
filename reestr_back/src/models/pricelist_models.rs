use crate::schema::dict_pricelist;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = dict_pricelist)]
pub struct PricelistItem {
    pub id: i32,
    pub name: String,
    pub price: BigDecimal,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = dict_pricelist)]
pub struct PricelistInsertDTO {
    pub name: String,
    pub price: BigDecimal,
}

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[diesel(table_name = dict_pricelist)]
pub struct PricelistUpdateDTO {
    pub id: i32,
    pub name: String,
    pub price: BigDecimal,
}
