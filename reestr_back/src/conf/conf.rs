use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub fn init_env() {
    dotenv().ok();
}

pub static DADATA_SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("DADATASECRETKEY").expect("DADATASECRETKEY not set"));
pub static DADATA_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("DADATAAPIKEY").expect("DADATAAPIKEY not set"));
pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL not set"));
