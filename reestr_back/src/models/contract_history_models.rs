use crate::schema::contract_history;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = contract_history)]
pub struct ContractHistory {
    pub id: i32,
    pub contract_id: i32,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = contract_history)]
pub struct ContractHistoryDTO {
    pub contract_id: i32,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: Option<String>,
}
