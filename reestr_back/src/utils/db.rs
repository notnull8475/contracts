use diesel::Connection;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    // Load environment variables from .env file if it exists
    // and establish a connection to the SQLite database specified in the environment variable.
    // If the environment variable is not set, it will panic with an error message.

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Establishing connection to {}", database_url);
    PgConnection::establish(&database_url).unwrap_or_else(|_| {
        log::error!("Error connecting to {}", database_url);
        panic!("Error connecting to {}", database_url)
    })
}
