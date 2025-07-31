use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
// Пользователь
#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub id: i32,
    pub login: String,
    pub username: String,
    pub role: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub login: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserDTO {
    pub login: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
}
// требования к авторизации
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,      // ID пользователя
    pub role: String, // Роль пользователя
    pub username: String,
    pub login: String,
    pub exp: usize, // Время истечения токена
}

// запрос на вход
#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
