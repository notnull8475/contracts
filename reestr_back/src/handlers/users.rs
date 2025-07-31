use crate::auth::auth::check_admin_token;
use crate::auth::roles::Role;
use crate::models::auth_models::{NewUser, NewUserDTO, User, UserDTO};
use crate::schema::users;
use crate::utils::db::establish_connection;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, web};
use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;

pub async fn add_user(
    req: HttpRequest,
    new_user_data: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    // Check JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Hash the password
    let hashed_password =
        hash(&new_user_data.password_hash, DEFAULT_COST).expect("Error hashing password");

    // Prepare the new user data
    let new_user = NewUserDTO {
        login: new_user_data.login.clone(),
        username: new_user_data.username.clone(),
        password_hash: hashed_password,
        role: new_user_data.role.clone(),
        is_active: true,
    };

    // Insert the new user into the database
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .expect("Error saving new user");

    Ok(HttpResponse::Ok().body("User added successfully"))
}

pub async fn get_users(req: HttpRequest) -> impl Responder {
    // Check JWT
    if let Some(response) = check_admin_token(&req) {
        return response;
    }

    let conn = &mut establish_connection();

    // Fetch users from the database
    let users: Vec<User> = users::table
        .select(users::all_columns)
        .get_results(conn)
        .expect("Error loading users");

    // Remove password hashes from the response
    let users_without_passwords: Vec<UserDTO> = users
        .into_iter()
        .map(|user: User| UserDTO {
            id: user.id,
            username: user.username,
            role: user.role,
            login: user.login,
            is_active: user.is_active,
        })
        .collect();
    info!("{:?}", users_without_passwords);
    HttpResponse::Ok().json(users_without_passwords)
}

pub async fn update_user(
    req: HttpRequest,
    user_data: web::Json<User>,
) -> Result<HttpResponse, Error> {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Хэширование пароля
    let hashed_password =
        hash(&user_data.password_hash, DEFAULT_COST).expect("Error hashing password");

    // Создаем обновленную структуру данных
    let updated_user = User {
        id: user_data.id,
        login: user_data.login.clone(),
        username: user_data.username.clone(),
        role: user_data.role.clone(),
        password_hash: hashed_password, // Используем хэшированный пароль
        is_active: user_data.is_active,
    };

    // Обновление пользователя
    let result = diesel::update(users::table.find(user_data.id))
        .set(&updated_user) // Передаем всю структуру
        .execute(conn)
        .expect("Error updating user");

    if result == 0 {
        return Ok(HttpResponse::NotFound().body("User not found"));
    }

    Ok(HttpResponse::Ok().body("User updated successfully"))
}
pub async fn get_roles(req: HttpRequest) -> impl Responder {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return response;
    }

    let all_roles = Role::all_roles_str();
    info!("{:?}", all_roles);
    HttpResponse::Ok().json(all_roles)
}

pub async fn del_user(req: HttpRequest, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Удаление пользователя
    let result = diesel::delete(users::table.filter(users::id.eq(*user_id)))
        .execute(conn)
        .expect("Error deleting user");

    if result == 0 {
        return Ok(HttpResponse::NotFound().body("User not found"));
    }

    Ok(HttpResponse::Ok().body("User deleted successfully"))
}

pub async fn get_user(_req: HttpRequest, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let user: User = users::table
        .filter(users::id.eq(user_id.into_inner()))
        .select(users::all_columns)
        .first(conn)
        .expect("User not found");
    let user_dto = UserDTO {
        id: user.id,
        login: user.login,
        username: user.username,
        role: user.role,
        is_active: user.is_active,
    };
    Ok(HttpResponse::Ok().json(user_dto))
}
