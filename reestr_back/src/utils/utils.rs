use actix_web::{Error, HttpResponse};

/// Подпись к внешнему ключу для сообщения об ошибке: пользователь видит
/// «не найдена организация», а не имя constraint из PostgreSQL.
fn fk_label(err_lower: &str) -> &'static str {
    const LABELS: [(&str, &str); 7] = [
        ("organization_fk", "организация"),
        ("resp_person_fk", "ответственное лицо"),
        ("type_of_validity_fk", "тип договора"),
        ("contract_status_id_fkey", "статус договора"),
        ("pricelist_id_fkey", "позиция прайса"),
        ("contract_fk_fkey", "договор"),
        ("supplementary_agreement", "доп. соглашение"),
    ];
    // «supplementary_agreement_contract_id_fkey» ссылается на договор, а не на соглашение.
    if err_lower.contains("supplementary_agreement_contract_id_fkey") {
        return "договор";
    }
    LABELS
        .iter()
        .find(|(needle, _)| err_lower.contains(needle))
        .map(|(_, label)| *label)
        .unwrap_or("связанная запись")
}

/// Переводит текст ошибки сервиса в HTTP-ответ.
///
/// Сервисы возвращают `Result<T, String>`, где строка содержит текст ошибки Diesel.
/// Раньше любое «violates» считалось попыткой удалить используемую запись, и при
/// добавлении договора с несуществующей организацией пользователь видел
/// «не может быть удален». Теперь ошибки различаются по тексту PostgreSQL.
pub fn error_response(err: &str) -> HttpResponse {
    let e = err.to_lowercase();
    let json = |msg: String| serde_json::json!({ "error": msg });

    let fk = e.contains("foreign key");
    if e.contains("is still referenced") || (fk && e.contains("update or delete")) {
        return HttpResponse::Conflict().json(json(
            "Объект используется в связанных данных и не может быть удален".to_string(),
        ));
    }
    if fk && e.contains("insert or update") {
        return HttpResponse::BadRequest().json(json(format!(
            "Указана несуществующая связанная запись: {}",
            fk_label(&e)
        )));
    }
    if e.contains("duplicate key") {
        return HttpResponse::Conflict()
            .json(json("Запись с таким значением уже существует".to_string()));
    }
    if e.contains("record not found") {
        return HttpResponse::NotFound().json(json(err.to_string()));
    }
    HttpResponse::BadRequest().json(json(err.to_string()))
}

pub fn response_fn<T: serde::Serialize>(resp: Result<T, String>) -> Result<HttpResponse, Error> {
    match resp {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(err) => Ok(error_response(&err)),
    }
}

/// Проверяет, что строка непустая после обрезки пробелов.
pub fn require_non_empty(value: &str, what: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{} не может быть пустым", what))
    } else {
        Ok(())
    }
}

/// Сумма не может быть отрицательной (цена договора, доп. соглашения, прайса).
pub fn require_non_negative(value: Option<&bigdecimal::BigDecimal>, what: &str) -> Result<(), String> {
    use bigdecimal::Signed;
    match value {
        Some(v) if v.is_negative() => Err(format!("{} не может быть отрицательной", what)),
        _ => Ok(()),
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
    use super::{empty_string_as_none_decimal, error_response, require_non_negative};
    use actix_web::http::StatusCode;
    use serde::Deserialize;

    fn status(err: &str) -> StatusCode {
        error_response(err).status()
    }

    #[test]
    fn удаление_используемой_записи_даёт_409() {
        let err = r#"Error to delete organization: update or delete on table "organization" violates foreign key constraint "organization_fk" on table "contract""#;
        assert_eq!(status(err), StatusCode::CONFLICT);
    }

    #[actix_web::test]
    async fn вставка_с_несуществующей_ссылкой_даёт_400_и_называет_поле() {
        let err = r#"Error to add contract: insert or update on table "contract" violates foreign key constraint "organization_fk""#;
        let resp = error_response(err);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text.contains("организация"), "{text}");
        assert!(!text.contains("удален"), "{text}");
    }

    #[test]
    fn не_найдено_даёт_404_а_дубль_409() {
        assert_eq!(status("Contract not found: Record not found"), StatusCode::NOT_FOUND);
        assert_eq!(
            status(r#"duplicate key value violates unique constraint "users_login_key""#),
            StatusCode::CONFLICT
        );
        assert_eq!(status("что-то иное"), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn отрицательная_сумма_отклоняется() {
        use std::str::FromStr;
        let neg = bigdecimal::BigDecimal::from_str("-1").unwrap();
        let zero = bigdecimal::BigDecimal::from_str("0").unwrap();
        assert!(require_non_negative(Some(&neg), "Цена").is_err());
        assert!(require_non_negative(Some(&zero), "Цена").is_ok());
        assert!(require_non_negative(None, "Цена").is_ok());
    }

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
