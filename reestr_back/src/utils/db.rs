use crate::conf::conf;
use diesel::pg::PgConnection;
use diesel::Connection;

pub fn establish_connection() -> PgConnection {
    // DATABASE_URL содержит пароль — в лог и в текст паники его не выводим.
    PgConnection::establish(&*conf::DATABASE_URL).unwrap_or_else(|e| {
        log::error!("Error connecting to database: {}", e);
        panic!("Error connecting to database: {}", e)
    })
}
