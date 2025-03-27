//! Handles the connection to the database
use std::{env, process};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::{debug_println, get_data_dir};

/// Diesel migrations constant
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// It returns the connection path as a String.
fn get_connection_string() -> String {
    if cfg!(debug_assertions) {
        format!("{}", env::var("DATABASE_URL").expect("Failed to get DATABASE_URL from .env file"))
    }
    else {
        format!("{}{}",
                get_data_dir(),
                env::var("DATABASE_URL").expect("Failed to get DATABASE_URL from .env file")
        )
    }
}

/// It stablishes an SQLite connection, runs the pending migrations and returns the connection itself.
pub fn stablish_and_run_migrations() -> SqliteConnection {
    dotenv::dotenv().ok();
    let conn = get_connection_string();
    debug_println!("connecting to {conn}");
    match SqliteConnection::establish(&conn) {
        Ok(mut conn) => {
            match conn.run_pending_migrations(MIGRATIONS) {
                Ok(_) => conn,
                Err(e) => {
                    eprintln!("Error running migrations: {e}");
                    process::exit(1);
                }
            }
        },
        Err(error) => {
            eprintln!("Unable to connect to database: {}", error);
            process::exit(1);
        }
    }
}

