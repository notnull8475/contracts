use crate::models::auth_models::User;
use crate::schema::users;
use crate::utils::db::establish_connection;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateSpecificationRequest {
    #[validate(length(min = 1, message = "Наименование характеристики не может быть пустой"))]
    pub name: String,
    pub is_positive: bool,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateSpecificationRequest {
    #[validate(length(min = 1, message = "Наименование характеристики не может быть пустой"))]
    pub name: String,
    pub is_positive: bool,
    pub description: String,
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateNewUserRequest {
    #[validate(length(min = 1, message = "Имя не может быть пустым"))]
    pub username: String,
    #[validate(length(min = 1, message = "Пароль не может быть пустым"))]
    pub password: String,
    #[validate(
        length(min = 1, message = "Логин не может быть пустым"),
        custom(function = "validate_unique_username")
    )]
    pub login: String,
}
fn validate_unique_username(login: &str) -> Result<(), ValidationError> {
    let conn = &mut establish_connection();
    if let Ok(_) = users::table
        .filter(users::login.eq(login))
        .first::<User>(conn)
    {
        return Err(ValidationError::new("Логин уже используется"));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateRatingOfUserRequest {
    #[validate(range(
        min = 0,
        max = 10,
        message = "Значение рейтинга не может быть вне диапазона от 1 до 10"
    ))]
    pub rating: i32,
    #[validate(range(min = 1))]
    pub specification_id: i32,
    #[validate(range(min = 1))]
    pub on_rate_user_id: i32,
    #[validate(range(min = 1))]
    pub is_rate_user_id: i32,
    pub is_rate_user_name: Option<String>,
    pub guest_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRatingRequest {
    pub id: i32,
    #[validate(range(
        min = 1,
        max = 10,
        message = "Значение рейтинга не может быть вне диапазона от 1 до 10"
    ))]
    pub rating: i32,
    #[validate(range(min = 1))]
    pub specification_id: i32,
    #[validate(range(min = 1))]
    pub on_rate_user_id: i32,
    #[validate(range(min = 1))]
    pub is_rate_user_id: i32,
    pub is_rate_user_name: String,
}
