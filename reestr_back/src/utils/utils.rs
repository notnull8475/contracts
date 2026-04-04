use actix_web::{Error, HttpResponse};

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
