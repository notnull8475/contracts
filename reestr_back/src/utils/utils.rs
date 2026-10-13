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

/// Десериализует необязательную сумму, принимая `""` как отсутствие значения.
///
/// Очищенное поле `<input type="number">` во Vue отдаёт пустую строку, а
/// `Option<BigDecimal>` на неё падает с «Failed to parse empty string» → 400.
pub fn empty_string_as_none_decimal<'de, D>(
    deserializer: D,
) -> Result<Option<bigdecimal::BigDecimal>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use std::str::FromStr;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Str(String),
        Num(serde_json::Number),
    }

    match Option::<Raw>::deserialize(deserializer)? {
        None => Ok(None),
        Some(Raw::Str(s)) if s.trim().is_empty() => Ok(None),
        Some(Raw::Str(s)) => bigdecimal::BigDecimal::from_str(s.trim())
            .map(Some)
            .map_err(serde::de::Error::custom),
        Some(Raw::Num(n)) => bigdecimal::BigDecimal::from_str(&n.to_string())
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::empty_string_as_none_decimal;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Dto {
        #[serde(default, deserialize_with = "empty_string_as_none_decimal")]
        price: Option<bigdecimal::BigDecimal>,
    }

    fn price(json: &str) -> Option<String> {
        serde_json::from_str::<Dto>(json)
            .expect("должно разбираться")
            .price
            .map(|p| p.to_string())
    }

    #[test]
    fn пустая_строка_и_null_дают_none() {
        assert_eq!(price(r#"{"price":""}"#), None);
        assert_eq!(price(r#"{"price":null}"#), None);
        assert_eq!(price(r#"{}"#), None);
    }

    #[test]
    fn число_и_строка_с_числом_разбираются() {
        assert_eq!(price(r#"{"price":1500.5}"#), Some("1500.5".to_string()));
        assert_eq!(price(r#"{"price":"1500.50"}"#), Some("1500.50".to_string()));
        assert_eq!(price(r#"{"price":12}"#), Some("12".to_string()));
    }

    #[test]
    fn мусор_даёт_ошибку() {
        assert!(serde_json::from_str::<Dto>(r#"{"price":"abc"}"#).is_err());
    }
}
