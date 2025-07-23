use crate::auth;
use crate::auth::auth::check_admin_token;
use crate::models::auth_models::{NewUser, User, UserWithoutPassword};
use crate::schema::users;
use crate::schema::users::name;
use crate::utils::db::establish_connection;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, web};
use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use std::string::String;

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
        hash(&new_user_data.password, DEFAULT_COST).expect("Error hashing password");

    // Prepare the new user data
    let new_user = NewUser {
        login: new_user_data.login.clone(),
        name: new_user_data.name.clone(),
        password: hashed_password,
        app_role_id: new_user_data.app_role_id,
    };

    // Insert the new user into the database
    diesel::insert_into(users::table)
        .values(&new_user)
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
    let users = users::table
        .load::<User>(conn)
        .expect("Error loading users");

    // Remove password hashes from the response
    let users_without_passwords: Vec<UserWithoutPassword> = users
        .into_iter()
        .map(|user| UserWithoutPassword {
            id: user.id,
            name: user.name,
            login: user.login,
        })
        .collect();
    println!("{:?}", users_without_passwords);

    HttpResponse::Ok().json(users_without_passwords)
}

// Define a new struct without password hash
pub async fn get_user_roles(req: HttpRequest, user_id: web::Path<i32>) -> impl Responder {
    if let Err(response) = auth::auth::verify_and_extract_claims(&req) {
        return response;
    }

    let conn = &mut establish_connection();

    // Fetch user roles from the database
    let roles: Vec<i32> = users::table
        .filter(users::id.eq(user_id.into_inner()))
        .select(users::app_role_id)
        .distinct()
        .load::<i32>(conn)
        .expect("Error loading user roles");

    HttpResponse::Ok().json(roles)
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
        hash(&user_data.password, DEFAULT_COST).expect("Error hashing password");

    // Создаем обновленную структуру данных
    let updated_user = NewUser {
        login: user_data.login.clone(),
        name: user_data.name.clone(),
        password: hashed_password, // Используем хэшированный пароль
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

    // Возможные роли системы
    let roles = vec!["admin", "manager", "employee"];

    HttpResponse::Ok().json(roles)
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

pub async fn get_user_name_by_id(
    _req: HttpRequest,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &mut establish_connection();
    let user_name: String = users::table
        .filter(users::id.eq(user_id.into_inner()))
        .select(name)
        .first(conn)
        .expect("User not found");
    Ok(HttpResponse::Ok().json(user_name))
}
