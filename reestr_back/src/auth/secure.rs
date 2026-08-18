use crate::auth::auth::create_jwt;
use crate::models::auth_models::{LoginRequest, LoginResponse, User};
use crate::schema::users;
use crate::utils::db::establish_connection;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub async fn login_user(login_data: web::Json<LoginRequest>) -> impl Responder {
    let conn = &mut establish_connection();

    log::info!("Logging in user: {:?}", login_data.login);

    if let Ok(user) = users::table
        .filter(users::login.eq(&login_data.login))
        .first::<User>(conn)
    {
        // Пустой пароль недопустим: иначе пользователь с пустым хешем входит без пароля.
        if login_data.password.is_empty() {
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }

        let bcrypt_match = verify(&login_data.password, &user.password_hash).unwrap_or(false);

        if bcrypt_match {
            if !user.is_active {
                log::warn!("Login denied for deactivated user: {}", user.login);
                return HttpResponse::Forbidden().body("User is deactivated");
            }

            let token = create_jwt(&user);
            return HttpResponse::Ok().json(LoginResponse { token });
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
