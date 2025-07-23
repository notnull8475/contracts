use crate::models::auth_models::{Claims, User};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ROLE_MAP: HashMap<i32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(1, "admin");
        map.insert(2, "moderator");
        map.insert(3, "user");
        map
    };
}

pub fn has_role(user: &User, role_name: &str) -> bool {
    get_role_name_by_id(user.app_role_id) == role_name
}

pub fn has_role_claims(claims: &Claims, role_name: &str) -> bool {
    get_role_name_by_id(claims.app_role_id) == role_name
}
pub fn get_role_name_by_id(role_id: i32) -> &'static str {
    ROLE_MAP.get(&role_id).copied().unwrap_or("unknown")
}
