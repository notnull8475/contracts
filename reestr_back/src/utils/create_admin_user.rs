use crate::auth::roles::Role;
use crate::models::auth_models::{NewUserDTO, User, UserDTO};
use crate::schema::users;
use crate::utils::db::establish_connection;
use bcrypt::{DEFAULT_COST, hash};
use diesel::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use log::{error, info};

pub fn create_admin_user_if_need() {
    let conn = &mut establish_connection();
    let user: Result<User, bool> = users::table
        .filter(users::role.eq(Role::Admin.as_str()))
        .select(users::all_columns)
        .first(conn)
        .map_err(|_| false);
    if user.is_err() {
        let user = NewUserDTO {
            login: "admin".to_string(),
            username: "admin".to_string(),
            password_hash: hash("admin", DEFAULT_COST).expect("Error hashing password"),
            role: Role::Admin.as_str().to_string(),
            is_active: true,
        };
        println!("Create admin user");
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .expect("Error saving new user");
        error!("Error saving new user");
    }
    info!("Create admin user registered successfully");
}
