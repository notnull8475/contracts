/// =============================================================================
/// import.rs — CLI-утилита импорта договоров из Excel в БД
/// =============================================================================
/// Запуск (из директории reestr_back/):
///   cargo run --bin import -- --file "../Реестр.xlsx"
///   cargo run --bin import -- --file "../Реестр.xlsx" --dry-run
///   cargo run --bin import -- --help
/// =============================================================================

use calamine::{open_workbook, Data, DataType, Reader, Xlsx};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Connection;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

// =============================================================================
// Diesel таблицы (inline, без crate::schema)
// =============================================================================
diesel::table! {
    contract (id) {
        id -> Int4,
        number -> Text,
        date_from -> Nullable<diesel::sql_types::Timestamp>,
        organization_id -> Int4,
        type_of_validity -> Nullable<Int4>,
        responsible_person_id -> Nullable<Int4>,
        address -> Nullable<Text>,
        additional_agreement -> Nullable<Text>,
        comment -> Nullable<Text>,
        date_to -> Nullable<diesel::sql_types::Timestamptz>,
        contract_period -> Nullable<Int4>,
        created_time -> Nullable<diesel::sql_types::Timestamptz>,
        updated_time -> Nullable<diesel::sql_types::Timestamptz>,
        actual -> Nullable<Bool>,
        file_link -> Nullable<Text>,
    }
}

diesel::table! {
    organization (id) {
        id -> Int4,
        short_name_with_opf -> Text,
        inn -> Int8,
        fact_address -> Nullable<Text>,
        legal_address -> Nullable<Text>,
        management_post -> Nullable<Text>,
        management_name -> Nullable<Text>,
        ogrn -> Nullable<Text>,
        full_name_with_opf -> Nullable<Text>,
        opf_full -> Nullable<Text>,
        opf_short -> Nullable<Text>,
    }
}

diesel::table! {
    responsible_person (id) {
        id -> Int4,
        firstname -> Text,
        name -> Text,
        lastname -> Nullable<Text>,
        user_id -> Nullable<Int4>,
    }
}

// =============================================================================
// Вставляемые DTO
// =============================================================================
#[derive(Insertable, Debug)]
#[diesel(table_name = contract)]
struct ContractImport {
    number: String,
    date_from: Option<NaiveDateTime>,
    organization_id: i32,
    responsible_person_id: Option<i32>,
    address: Option<String>,
    comment: Option<String>,
    actual: Option<bool>,
    file_link: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = organization)]
struct OrgInsert {
    short_name_with_opf: String,
    inn: i64,
    fact_address: Option<String>,
    full_name_with_opf: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = responsible_person)]
struct PersonInsert {
    lastname: Option<String>,
    firstname: String,
    name: String,
}

// =============================================================================
// Статистика импорта
// =============================================================================
#[derive(Default)]
struct Stats {
    contracts: usize,
    duplicates: usize,
    orgs: usize,
    persons: usize,
    skipped: usize,
    total: usize,
}

// =============================================================================
// Нормализация строки для нечёткого сравнения
// =============================================================================
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// =============================================================================
// Парсинг даты из ячейки Excel
// =============================================================================
fn parse_date(cell: &Data) -> Option<NaiveDateTime> {
    match cell {
        Data::DateTime(dt) => dt.as_datetime(),

        Data::DateTimeIso(s) => {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| {
                    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                })
                .ok()
        }

        Data::Float(f) => {
            // Иногда дата хранится как f64 serial в xlsx
            calamine::ExcelDateTime::new(*f, calamine::ExcelDateTimeType::DateTime, false)
                .as_datetime()
        }

        Data::String(s) => parse_date_str(s.trim()),

        _ => None,
    }
}

fn parse_date_str(s: &str) -> Option<NaiveDateTime> {
    if s.is_empty() || s.contains('?') || s.starts_with('_') {
        return None;
    }

    // Убираем суффиксы: "г", "г."
    let s = s
        .trim_end_matches(',')
        .trim_end_matches('.')
        .trim_end_matches('г')
        .trim();

    let formats = ["%d.%m.%Y", "%d.%m.%y", "%Y-%m-%d"];
    for fmt in formats {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(s, fmt) {
            return Some(d.and_hms_opt(0, 0, 0)?);
        }
    }
    None
}

// =============================================================================
// Проверка: является ли строка строкой данных (есть номер договора)
// =============================================================================
fn extract_number(row: &[Data]) -> Option<String> {
    let cell = row.first()?;
    let s = match cell {
        Data::String(s) => s.trim().to_string(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        _ => return None,
    };

    if s.is_empty() {
        return None;
    }

    // Строки-разделители: заголовки, названия годов
    let lower = s.to_lowercase();
    if lower.contains("реестр")
        || lower.starts_with("№")
        || lower.starts_with("n ")
        || (s.len() <= 6 && s.chars().all(|c| c.is_numeric() || c == 'г' || c == ' '))
    {
        return None;
    }

    Some(s)
}

// =============================================================================
// Найти или создать организацию
// =============================================================================
fn find_or_create_org(
    conn: &mut PgConnection,
    name: &str,
    address: Option<&str>,
    cache: &mut HashMap<String, i32>,
    stats: &mut Stats,
    dry_run: bool,
) -> Option<i32> {
    if name.trim().is_empty() {
        return None;
    }

    let key = normalize(name);

    if let Some(&id) = cache.get(&key) {
        return Some(id);
    }

    // Поиск в БД
    let found = organization::table
        .select((organization::id, organization::short_name_with_opf))
        .load::<(i32, String)>(conn)
        .unwrap_or_default()
        .into_iter()
        .find(|(_, db_name)| normalize(db_name) == key);

    if let Some((id, _)) = found {
        cache.insert(key, id);
        return Some(id);
    }

    if dry_run {
        eprintln!("  [dry-run] +org: {}", name);
        let tmp_id = -((cache.len() as i32) + 1);
        cache.insert(key, tmp_id);
        return Some(tmp_id);
    }

    let inn_placeholder: i64 = 9_000_000_000 + cache.len() as i64;

    let dto = OrgInsert {
        short_name_with_opf: name.trim().to_string(),
        inn: inn_placeholder,
        fact_address: address.map(|s| s.trim().to_string()),
        full_name_with_opf: Some(name.trim().to_string()),
    };

    match diesel::insert_into(organization::table)
        .values(&dto)
        .returning(organization::id)
        .get_result::<i32>(conn)
    {
        Ok(id) => {
            cache.insert(key, id);
            stats.orgs += 1;
            Some(id)
        }
        Err(e) => {
            eprintln!("  [warn] org '{}': {}", name, e);
            None
        }
    }
}

// =============================================================================
// Найти или создать ответственного
// =============================================================================
fn find_or_create_person(
    conn: &mut PgConnection,
    full_name: &str,
    cache: &mut HashMap<String, i32>,
    stats: &mut Stats,
    dry_run: bool,
) -> Option<i32> {
    let full_name = full_name.trim();
    if full_name.is_empty() {
        return None;
    }

    let parts: Vec<&str> = full_name.splitn(3, ' ').collect();
    let lastname = parts.first().copied().unwrap_or("").trim().to_string();
    let firstname = parts.get(1).copied().unwrap_or("").trim().to_string();
    let middlename = parts.get(2).copied().unwrap_or("").trim().to_string();

    if lastname.is_empty() {
        return None;
    }

    let key = normalize(&lastname);

    if let Some(&id) = cache.get(&key) {
        return Some(id);
    }

    // Поиск по lastname
    let found = responsible_person::table
        .filter(responsible_person::lastname.is_not_null())
        .select((responsible_person::id, responsible_person::lastname))
        .load::<(i32, Option<String>)>(conn)
        .unwrap_or_default()
        .into_iter()
        .find(|(_, ln)| ln.as_deref().map(normalize).as_deref() == Some(key.as_str()));

    if let Some((id, _)) = found {
        cache.insert(key, id);
        return Some(id);
    }

    if dry_run {
        eprintln!("  [dry-run] +person: {}", full_name);
        let tmp_id = -((cache.len() as i32) + 1);
        cache.insert(key, tmp_id);
        return Some(tmp_id);
    }

    let dto = PersonInsert {
        lastname: Some(lastname.clone()),
        firstname,
        name: middlename,
    };

    match diesel::insert_into(responsible_person::table)
        .values(&dto)
        .returning(responsible_person::id)
        .get_result::<i32>(conn)
    {
        Ok(id) => {
            cache.insert(key, id);
            stats.persons += 1;
            Some(id)
        }
        Err(e) => {
            eprintln!("  [warn] person '{}': {}", full_name, e);
            None
        }
    }
}

// =============================================================================
// Обработка одного листа
// =============================================================================
fn process_sheet(
    sheet_name: &str,
    rows: &[Vec<Data>],
    conn: &mut PgConnection,
    org_cache: &mut HashMap<String, i32>,
    person_cache: &mut HashMap<String, i32>,
    stats: &mut Stats,
    dry_run: bool,
) {
    // Лист3 — расторгнутые → actual = false
    let actual = sheet_name != "Лист3";

    println!(
        "\n--- {} ({}) ---",
        sheet_name,
        if actual { "актуальные" } else { "закрытые" }
    );

    for row in rows {
        stats.total += 1;

        let number_raw = match extract_number(row) {
            Some(n) => n,
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // B — дата
        let date_from = row.get(1).and_then(|c| parse_date(c));

        // C — организация
        let org_name = row
            .get(2)
            .and_then(|c| c.as_string())
            .unwrap_or_default();
        let org_name = org_name.trim();

        // D — срок / комментарий
        let term_str = row
            .get(3)
            .and_then(|c| c.as_string())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        // E — ответственный (проверяем что это не дата/статус договора)
        let responsible_raw = row
            .get(4)
            .and_then(|c| c.as_string())
            .map(|s| s.trim().to_string())
            .filter(|s| {
                // Если в ячейке Е текст похожий на статус (расторгнут, закрыт, перезаключен)
                // или дата — это сдвиг колонок, не ответственный
                !s.to_lowercase().contains("расторг")
                    && !s.to_lowercase().contains("закрыт")
                    && !s.to_lowercase().contains("перезакл")
                    && !s.to_lowercase().contains("бессрочн")
                    && !s.to_lowercase().contains("год")
                    && !s.is_empty()
            })
            .unwrap_or_default();

        // F — гиперссылка на старый файл
        let file_link = row
            .get(5)
            .and_then(|c| c.as_string())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        // G (индекс 6) — адрес, на Лист1 бывает в индексе 7
        let address = {
            let col6 = row.get(6).and_then(|c| c.as_string()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            let col7 = row.get(7).and_then(|c| c.as_string()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            // Берём тот, который выглядит как адрес (содержит г., ул., д.)
            let looks_like_address = |s: &String| {
                s.contains("г.") || s.contains("ул.") || s.contains(" д.")
                    || s.contains("пр.") || s.contains("пр-т")
            };
            match (col6, col7) {
                (Some(a), _) if looks_like_address(&a) => Some(a),
                (_, Some(b)) if looks_like_address(&b) => Some(b),
                (Some(a), _) => Some(a),
                (None, Some(b)) => Some(b),
                _ => None,
            }
        };

        // --- Организация ---
        let effective_org = if org_name.is_empty() { "Не указано" } else { org_name };
        let org_id = match find_or_create_org(
            conn,
            effective_org,
            address.as_deref(),
            org_cache,
            stats,
            dry_run,
        ) {
            Some(id) if id > 0 => id,
            _ if dry_run => 0,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        // --- Ответственный ---
        let person_id = if responsible_raw.trim().is_empty() {
            None
        } else {
            find_or_create_person(conn, responsible_raw.trim(), person_cache, stats, dry_run)
                .filter(|&id| id > 0)
        };

        // --- Дубликат номера ---
        let (final_number, is_dup) = if !dry_run {
            use diesel::dsl::exists;
            use diesel::select;
            let dup = select(exists(
                contract::table.filter(contract::number.eq(&number_raw)),
            ))
            .get_result::<bool>(conn)
            .unwrap_or(false);
            if dup {
                (format!("{} (дубл)", number_raw), true)
            } else {
                (number_raw.clone(), false)
            }
        } else {
            (number_raw.clone(), false)
        };

        // --- Комментарий (срок + пометка дублика) ---
        let mut comments: Vec<String> = Vec::new();
        if let Some(t) = &term_str {
            comments.push(t.clone());
        }
        if is_dup {
            comments.push("(импорт: дубликат номера)".to_string());
        }
        let comment = if comments.is_empty() { None } else { Some(comments.join("; ")) };

        let dto = ContractImport {
            number: final_number.clone(),
            date_from,
            organization_id: org_id,
            responsible_person_id: person_id,
            address,
            comment,
            actual: Some(actual),
            file_link,
        };

        let date_str = date_from
            .map(|d| d.format("%d.%m.%Y").to_string())
            .unwrap_or_else(|| "—".to_string());

        if dry_run {
            println!(
                "  [dry] {:<25} | {:<40} | {}",
                final_number,
                effective_org.chars().take(40).collect::<String>(),
                date_str
            );
            stats.contracts += 1;
            continue;
        }

        match diesel::insert_into(contract::table)
            .values(&dto)
            .execute(conn)
        {
            Ok(_) => {
                let marker = if is_dup { " [дубл]" } else { "" };
                println!(
                    "  [ok] {}{} | {} | {}",
                    final_number, marker, effective_org, date_str
                );
                stats.contracts += 1;
                if is_dup {
                    stats.duplicates += 1;
                }
            }
            Err(e) => {
                eprintln!("  [err] {} — {}", final_number, e);
                stats.skipped += 1;
            }
        }
    }
}

// =============================================================================
// main
// =============================================================================
fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Использование:");
        println!("  cargo run --bin import -- --file <path.xlsx> [--dry-run]");
        println!();
        println!("  --file <path>   Путь к Excel-файлу (.xlsx)");
        println!("  --dry-run       Предпросмотр без записи в БД");
        return;
    }

    let dry_run = args.iter().any(|a| a == "--dry-run");

    let file_path = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| {
            std::fs::read_dir(".")
                .ok()?
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.path().extension().map(|x| x == "xlsx").unwrap_or(false)
                })
                .map(|e| e.path().to_string_lossy().to_string())
        })
        .unwrap_or_else(|| {
            eprintln!("Укажите файл: --file path/to/file.xlsx");
            std::process::exit(1);
        });

    println!("{}", "=".repeat(52));
    println!("Импорт договоров из Excel");
    println!("Файл: {}", file_path);
    if dry_run {
        println!("Режим: DRY-RUN (запись в БД отключена)");
    }
    println!("{}", "=".repeat(52));

    let mut workbook: Xlsx<_> = open_workbook(&file_path).unwrap_or_else(|e| {
        eprintln!("Ошибка открытия '{}': {}", file_path, e);
        std::process::exit(1);
    });

    let sheet_names: Vec<String> = workbook.sheet_names().to_vec();
    println!("Листов: {}", sheet_names.len());

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL не задан в .env");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|e| {
            eprintln!("Ошибка подключения к БД: {}", e);
            std::process::exit(1);
        });

    let mut org_cache: HashMap<String, i32> = HashMap::new();
    let mut person_cache: HashMap<String, i32> = HashMap::new();
    let mut stats = Stats::default();

    // Пропускаем лист "Доп соглашения" — он не содержит договоров
    let skip_sheets = ["Доп соглашения  к лиц договору"];

    for sheet_name in sheet_names.clone() {
        if skip_sheets.contains(&sheet_name.as_str()) {
            println!("\n--- {} [пропущен] ---", sheet_name);
            continue;
        }

        let rows: Vec<Vec<Data>> = match workbook.worksheet_range(&sheet_name) {
            Ok(range) => range.rows().map(|r| r.to_vec()).collect(),
            Err(e) => {
                eprintln!("Ошибка чтения '{}': {}", sheet_name, e);
                continue;
            }
        };

        process_sheet(
            &sheet_name,
            &rows,
            &mut conn,
            &mut org_cache,
            &mut person_cache,
            &mut stats,
            dry_run,
        );
    }

    println!("\n{}", "=".repeat(52));
    println!("Итоги импорта:");
    println!("  ✓ Договоров создано:         {}", stats.contracts);
    println!("  ✓ Организаций создано:       {}", stats.orgs);
    println!("  ✓ Ответственных создано:     {}", stats.persons);
    println!("  ! Дубликатов (с пометкой):   {}", stats.duplicates);
    println!("  ! Строк пропущено:           {}", stats.skipped);
    println!("  ~ Строк обработано:          {}", stats.total);
    println!("{}", "=".repeat(52));
}
