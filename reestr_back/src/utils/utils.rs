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
        Err(err) => {
            let err_lower = err.to_lowercase();

            if err_lower.contains("foreign key")
                || err_lower.contains("violates")
                || err_lower.contains("is still referenced")
            {
                return Ok(HttpResponse::Conflict()
                    .json(serde_json::json!({"error": "Объект используется в связанных данных и не может быть удален"})));
            }

            Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": err})))
        }
    }
}
