use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub fn init_env() {
    dotenv().ok();
}

/// Секрет для подписи JWT и производных ключей (минимум 32 символа).
pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    let s = env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment");
    assert!(
        s.len() >= 32,
        "JWT_SECRET must be at least 32 characters for adequate security"
    );
    s
});

pub static DADATA_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("DADATAAPIKEY").expect("DADATAAPIKEY not set"));
pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL not set"));
