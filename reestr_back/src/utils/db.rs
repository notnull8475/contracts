use crate::conf::conf;
use diesel::pg::PgConnection;
use diesel::Connection;
pub fn establish_connection() -> PgConnection {
    // Load environment variables from .env file if it exists
    // and establish a connection to the SQLite database specified in the environment variable.
    // If the environment variable is not set, it will panic with an error message.

    log::info!("Establishing connection to {}", *conf::DATABASE_URL);
    PgConnection::establish(&*conf::DATABASE_URL).unwrap_or_else(|_| {
        log::error!("Error connecting to {}", *conf::DATABASE_URL);
        panic!("Error connecting to {}", *conf::DATABASE_URL)
    })
}
