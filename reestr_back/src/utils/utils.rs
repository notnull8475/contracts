use actix_web::{Error, HttpResponse};
use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn response_fn<T: serde::Serialize>(resp: Result<T, String>) -> Result<HttpResponse, Error> {
    match resp {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(err) => Ok(HttpResponse::Unauthorized().body(err)),
    }
}
