use crate::models::auth_models::{Claims, User};

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Moderator,
    User,
    Unknown,
}

impl Role {
    pub fn all_roles() -> Vec<Self> {
        vec![Role::Admin, Role::Moderator, Role::User]
    }
    pub fn all_roles_str() -> Vec<&'static str> {
        vec![
            Role::Admin.as_str(),
            Role::Moderator.as_str(),
            Role::User.as_str(),
        ]
    }
    /// Преобразует текстовое представление роли в значение перечисления Role.
    pub fn from_str(role_name: &str) -> Self {
        match role_name {
            "admin" => Role::Admin,
            "moderator" => Role::Moderator,
            "user" => Role::User,
            _ => Role::Unknown,
        }
    }

    /// Возвращает строковое представление роли.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Moderator => "moderator",
            Role::User => "user",
            Role::Unknown => "unknown",
        }
    }
}

/// Проверяет, имеет ли пользователь указанную роль.
pub fn has_role(user: &User, role_name: &str) -> bool {
    let user_role = Role::from_str(&user.role); // Предполагается, что у User есть поле role_name: String
    user_role.as_str() == role_name
}

/// Проверяет, имеет ли claims указанную роль.
pub fn has_role_claims(claims: &Claims, role_name: &str) -> bool {
    let claims_role = Role::from_str(&claims.role); // Предполагается, что у Claims есть поле role_name: String
    claims_role.as_str() == role_name
}

/// Возвращает строковое представление роли по её имени.
pub fn get_role_name_by_str(role_name: &str) -> &'static str {
    Role::from_str(role_name).as_str()
}
