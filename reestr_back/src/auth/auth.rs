use crate::auth::roles::has_role_claims;
use crate::conf::conf::JWT_SECRET;
use crate::models::auth_models::{Claims, User};
use crate::schema::users;
use crate::utils::db::establish_connection;
use actix_web::cookie::Key;
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sha2::{Digest, Sha512};

pub fn cookie_session_key() -> Key {
    let mut h = Sha512::new();
    h.update(b"actix-session-cookie-v1:");
    h.update(JWT_SECRET.as_bytes());
    let digest = h.finalize();
    let arr: [u8; 64] = digest.into();
    Key::from(&arr)
}

pub fn create_jwt(user: &User) -> String {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        id: user.id,
        login: user.login.to_string(),
        username: user.username.to_string(),
        exp: expiration.timestamp() as usize,
        role: user.role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("JWT encoding failed")
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}

fn bearer_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|t| t.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|t| !t.is_empty())
}

/// Актуальные роль и признак активности из БД.
///
/// Токен живёт 24 часа и несёт роль на момент выдачи, поэтому одной проверки
/// подписи мало: отключённый или разжалованный пользователь иначе сохранял бы
/// прежний доступ до истечения токена.
fn current_account(user_id: i32) -> Option<(String, bool)> {
    let conn = &mut establish_connection();
    users::table
        .find(user_id)
        .select((users::role, users::is_active))
        .first::<(String, bool)>(conn)
        .ok()
}

/// Проверяет токен и сверяет его с текущим состоянием учётной записи.
fn authenticate(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let Some(token) = bearer_token(req) else {
        return Err(HttpResponse::Unauthorized().body("Missing or invalid token"));
    };

    let Some(mut claims) = verify_jwt(token) else {
        return Err(HttpResponse::Unauthorized().body("Invalid token"));
    };

    let Some((role, is_active)) = current_account(claims.id) else {
        return Err(HttpResponse::Unauthorized().body("Invalid token: account not found"));
    };

    if !is_active {
        return Err(HttpResponse::Forbidden().body("User is deactivated"));
    }

    // Роль берём из БД, а не из токена: разжалование действует немедленно.
    claims.role = role;
    Ok(claims)
}

pub fn verify_and_extract_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    authenticate(req)
}

pub(crate) fn check_admin_token(req: &HttpRequest) -> Option<HttpResponse> {
    match authenticate(req) {
        Err(response) => Some(response),
        Ok(claims) => {
            if has_role_claims(&claims, "admin") {
                None
            } else {
                Some(HttpResponse::Forbidden().body("Access denied: admin role required"))
            }
        }
    }
}
