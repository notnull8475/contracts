use crate::auth::auth::create_jwt;
use crate::models::auth_models::{LoginRequest, LoginResponse, NewUserDTO, User};
use crate::schema::users;
use crate::utils::db::establish_connection;
use actix_web::{HttpResponse, Responder, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::RunQueryDsl;
use diesel::prelude::*;

// Создание нового пользователя
pub async fn register_user(user_data: web::Json<NewUserDTO>) -> impl Responder {
    let conn = &mut establish_connection();

    log::info!("Registering user: {:?}", user_data.login);

    let hashed_password =
        hash(&user_data.password_hash, DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUserDTO {
        login: user_data.login.clone(),
        password_hash: hashed_password,
        role: "".to_string(),
        username: user_data.username.clone(), // Add the missing `name` field
        is_active: true,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    HttpResponse::Ok().body("User registered successfully")
}

// Авторизация
#[actix_web::post("/api/v1/login")]
pub async fn login_user(login_data: web::Json<LoginRequest>) -> impl Responder {
    let conn = &mut establish_connection();

    log::info!("Logging in user: {:?}", login_data.login);

    if let Ok(user) = users::table
        .filter(users::login.eq(&login_data.login))
        .first::<User>(conn)
    {
        if verify(&login_data.password, &user.password_hash).unwrap_or(false) {
            let token = create_jwt(&user);
            return HttpResponse::Ok().json(LoginResponse { token });
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
// pub async fn logout(user: web::Json<User>) -> impl Responder {
//     let conn = &mut establish_connection();
//     log::info!("Logging out user: {:?}", user.login);
//
// }
