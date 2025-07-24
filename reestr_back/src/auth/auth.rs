use crate::auth::roles::has_role_claims;
use crate::models::auth_models::{Claims, User};
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

const SECRET_KEY: &[u8] = b"zG1wHJNloHwaJZMdHxQlZEGyElUPyo7QD7smArPruyOlKFxViMoO710";

pub fn create_jwt(user: &User) -> String {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        id: user.id,
        login: user.login.to_string(),
        username: user.login.to_string(),
        exp: expiration.timestamp() as usize,
        role: user.role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .unwrap()
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}
pub fn verify_and_extract_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .filter(|t| t.starts_with("Bearer "))
        .map(|t| t.trim_start_matches("Bearer "));

    if let Some(token) = token {
        if let Some(claims) = verify_jwt(token) {
            return Ok(claims);
        }
    }

    Err(HttpResponse::Unauthorized().body("Missing or invalid token"))
}
pub(crate) fn check_admin_token(req: &HttpRequest) -> Option<HttpResponse> {
    // Извлекаем заголовок Authorization
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .filter(|t| t.starts_with("Bearer "))
        .map(|t| t.trim_start_matches("Bearer ").trim());

    // Если токен отсутствует или неверного формата
    if token.is_none() {
        return Some(HttpResponse::Unauthorized().body("Missing or invalid token"));
    }

    // Проверяем JWT
    let claims = verify_jwt(token.unwrap());
    if claims.is_none() {
        return Some(HttpResponse::Unauthorized().body("Invalid token"));
    }

    // Проверяем роль пользователя
    let claims = claims.unwrap();
    if !has_role_claims(&claims, "admin") {
        return Some(HttpResponse::Forbidden().body("Access denied: admin role required"));
    }

    None
}
