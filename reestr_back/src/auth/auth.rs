use crate::auth::roles::has_role_claims;
use crate::conf::conf::JWT_SECRET;
use crate::models::auth_models::{Claims, User};
use actix_web::cookie::Key;
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
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

pub fn verify_and_extract_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .filter(|t| t.starts_with("Bearer "))
        .map(|t| t.trim_start_matches("Bearer ").trim());

    if let Some(token) = token {
        if let Some(claims) = verify_jwt(token) {
            return Ok(claims);
        }
    }

    Err(HttpResponse::Unauthorized().body("Missing or invalid token"))
}

pub(crate) fn check_admin_token(req: &HttpRequest) -> Option<HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .filter(|t| t.starts_with("Bearer "))
        .map(|t| t.trim_start_matches("Bearer ").trim());

    let token = match token {
        Some(t) => t,
        None => return Some(HttpResponse::Unauthorized().body("Missing or invalid token")),
    };

    let claims = match verify_jwt(token) {
        Some(c) => c,
        None => return Some(HttpResponse::Unauthorized().body("Invalid token")),
    };

    if !has_role_claims(&claims, "admin") {
        return Some(HttpResponse::Forbidden().body("Access denied: admin role required"));
    }

    None
}
