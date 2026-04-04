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
        let bcrypt_match = verify(&login_data.password, &user.password_hash).unwrap_or(false);

        if bcrypt_match {
            let token = create_jwt(&user);
            return HttpResponse::Ok().json(LoginResponse { token });
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
