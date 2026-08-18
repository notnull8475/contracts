use crate::auth::auth::check_admin_token;
use crate::auth::roles::Role;
use crate::models::auth_models::{NewUser, NewUserDTO, UpdateUserRequest, User, UserDTO};
use crate::schema::users;
use crate::utils::db::establish_connection;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;

/// Ошибка нарушения уникальности логина -> понятное сообщение вместо 500.
fn db_error_response(err: diesel::result::Error, context: &str) -> HttpResponse {
    if let diesel::result::Error::DatabaseError(
        diesel::result::DatabaseErrorKind::UniqueViolation,
        _,
    ) = err
    {
        return HttpResponse::Conflict()
            .json(serde_json::json!({"error": "Пользователь с таким логином уже существует"}));
    }
    log::error!("{}: {}", context, err);
    HttpResponse::InternalServerError().json(serde_json::json!({"error": context}))
}

enum DeleteOutcome {
    Deleted,
    NotFound,
    LastAdmin,
}

/// `true`, если `user_id` — администратор и других администраторов в системе нет.
/// Вызывать внутри транзакции, когда результат влияет на последующую запись.
fn last_admin_check(
    conn: &mut diesel::PgConnection,
    user_id: i32,
) -> Result<bool, diesel::result::Error> {
    let is_admin: i64 = users::table
        .filter(users::id.eq(user_id))
        .filter(users::role.eq(Role::Admin.as_str()))
        .count()
        .get_result(conn)?;

    if is_admin == 0 {
        return Ok(false);
    }

    let admin_count: i64 = users::table
        .filter(users::role.eq(Role::Admin.as_str()))
        .count()
        .get_result(conn)?;

    Ok(admin_count <= 1)
}

fn validate_role(role: &str) -> Result<(), HttpResponse> {
    if Role::all_roles_str().contains(&role) {
        Ok(())
    } else {
        Err(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": format!("Неизвестная роль: {}", role)})))
    }
}

pub async fn add_user(
    req: HttpRequest,
    new_user_data: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    // Check JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    if new_user_data.login.trim().is_empty() {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "Логин не может быть пустым"})));
    }
    if new_user_data.password_hash.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "Пароль не может быть пустым"})));
    }
    if let Err(response) = validate_role(&new_user_data.role) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Hash the password
    let hashed_password = match hash(&new_user_data.password_hash, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            log::error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Ошибка хеширования пароля"})));
        }
    };

    // Prepare the new user data
    let new_user = NewUserDTO {
        login: new_user_data.login.trim().to_string(),
        username: new_user_data.username.clone(),
        password_hash: hashed_password,
        role: new_user_data.role.clone(),
        is_active: Some(new_user_data.is_active.unwrap_or(true)),
    };

    // Insert the new user into the database
    match diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
    {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Пользователь создан"}))),
        Err(e) => Ok(db_error_response(e, "Ошибка создания пользователя")),
    }
}

pub async fn get_users(req: HttpRequest) -> Result<HttpResponse, Error> {
    // Check JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Fetch users from the database
    let users: Vec<User> = match users::table
        .select(users::all_columns)
        .order(users::id.asc())
        .get_results(conn)
    {
        Ok(rows) => rows,
        Err(e) => return Ok(db_error_response(e, "Ошибка получения списка пользователей")),
    };

    // Remove password hashes from the response
    let users_without_passwords: Vec<UserDTO> = users
        .into_iter()
        .map(|user: User| UserDTO {
            id: user.id,
            username: user.username,
            role: user.role,
            login: user.login,
            is_active: Some(user.is_active),
        })
        .collect();
    info!("{:?}", users_without_passwords);
    Ok(HttpResponse::Ok().json(users_without_passwords))
}

pub async fn update_user(
    req: HttpRequest,
    user_data: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let user_data = user_data.into_inner();

    if user_data.login.trim().is_empty() {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "Логин не может быть пустым"})));
    }
    if let Err(response) = validate_role(&user_data.role) {
        return Ok(response);
    }

    let conn = &mut establish_connection();

    // Нельзя разжаловать или отключить последнего администратора: иначе система
    // останется без доступа к /api/v1/admin/* сразу после истечения текущего токена.
    let loses_admin_rights =
        user_data.role != Role::Admin.as_str() || user_data.is_active == Some(false);
    if loses_admin_rights {
        match last_admin_check(conn, user_data.id) {
            Ok(true) => {
                return Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Нельзя разжаловать или отключить последнего администратора"
                })));
            }
            Ok(false) => {}
            Err(e) => return Ok(db_error_response(e, "Ошибка проверки прав администратора")),
        }
    }

    // Обновляем профиль без пароля: пустое поле пароля в форме не должно его затирать.
    let profile = UserDTO {
        id: user_data.id,
        login: user_data.login.trim().to_string(),
        username: user_data.username.clone(),
        role: user_data.role.clone(),
        is_active: user_data.is_active,
    };

    // Пароль меняем только если он реально передан и не пустой.
    let new_password = user_data
        .password_hash
        .as_deref()
        .map(str::trim)
        .filter(|p| !p.is_empty());

    let hashed_password = match new_password.map(|p| hash(p, DEFAULT_COST)).transpose() {
        Ok(h) => h,
        Err(e) => {
            log::error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Ошибка хеширования пароля"})));
        }
    };

    // Профиль и пароль пишем одной транзакцией: частично применённое обновление
    // оставило бы учётную запись переименованной при сообщении об ошибке.
    let updated = conn.transaction::<usize, diesel::result::Error, _>(|conn| {
        let rows = diesel::update(users::table.find(user_data.id))
            .set(&profile)
            .execute(conn)?;

        if rows > 0 {
            if let Some(ref hashed) = hashed_password {
                diesel::update(users::table.find(user_data.id))
                    .set(users::password_hash.eq(hashed))
                    .execute(conn)?;
            }
        }

        Ok(rows)
    });

    let result = match updated {
        Ok(n) => n,
        Err(e) => return Ok(db_error_response(e, "Ошибка обновления пользователя")),
    };

    if result == 0 {
        return Ok(
            HttpResponse::NotFound().json(serde_json::json!({"error": "Пользователь не найден"}))
        );
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Пользователь обновлён"})))
}

pub async fn get_roles(req: HttpRequest) -> Result<HttpResponse, Error> {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let all_roles = Role::all_roles_str();
    info!("{:?}", all_roles);
    Ok(HttpResponse::Ok().json(all_roles))
}

pub async fn del_user(req: HttpRequest, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // Проверка JWT
    if let Some(response) = check_admin_token(&req) {
        return Ok(response);
    }

    let user_id = user_id.into_inner();
    let conn = &mut establish_connection();

    // Проверка «последнего администратора» и удаление — в одной транзакции,
    // иначе два параллельных запроса могут удалить обоих оставшихся админов.
    let outcome = conn.transaction::<DeleteOutcome, diesel::result::Error, _>(|conn| {
        if last_admin_check(conn, user_id)? {
            return Ok(DeleteOutcome::LastAdmin);
        }

        let rows = diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)?;
        Ok(if rows == 0 {
            DeleteOutcome::NotFound
        } else {
            DeleteOutcome::Deleted
        })
    });

    match outcome {
        Ok(DeleteOutcome::Deleted) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Пользователь удалён"})))
        }
        Ok(DeleteOutcome::NotFound) => Ok(
            HttpResponse::NotFound().json(serde_json::json!({"error": "Пользователь не найден"}))
        ),
        Ok(DeleteOutcome::LastAdmin) => Ok(HttpResponse::Conflict()
            .json(serde_json::json!({"error": "Нельзя удалить последнего администратора"}))),
        Err(e) => Ok(db_error_response(e, "Ошибка удаления пользователя")),
    }
}
