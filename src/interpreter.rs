//! # Main interpreter
//! This module interprets the arguments given by the user and delegates
//! the work to command submodules ([plan], [subject], [export]). It also provides
//! useful functions to every command submodule.

use crate::entry::Mode;
use crate::{debug_println, entry, plan, subject, usage};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::path::Path;
use std::{env, fs, process};

/// Diesel migrations constant
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_data_dir() -> String {
    #[cfg(target_os = "windows")]
    let home = format!(
        "{}\\.student_datahub\\",
        env::var("APPDATA").expect("Failed to get HOME environment variable")
    );

    #[cfg(not(target_os = "windows"))]
    let home = format!(
        "{}/.student_datahub/",
        env::var("HOME").expect("Failed to get HOME environment variable")
    );

    // We create the path if it doesn't exist.
    let path = Path::new(&home);
    if !path.exists() {
        fs::create_dir_all(path).expect("No se pudo crear la carpeta");
    }

    home
}

/// Interprets the first command of the arguments provided and delegates the work to submodule commands
/// # Arguments
/// * `args` - Program arguments.
pub fn interpret(args: &mut Vec<String>) {
    match args.len() {
        0 => {
            usage::display_usage();
            process::exit(0);
        }
        _ => {
            let option = args.get(0).unwrap().clone();
            args.remove(0);
            debug_println!("using arg: {option}");
            dotenv::dotenv().ok();
            let connection_string = format!(
                "{}{}",
                get_data_dir(),
                env::var("DATABASE_URL").expect("Failed to get DATABASE_URL from .env file")
            );
            debug_println!("connecting to {connection_string}");
            let mut conn = SqliteConnection::establish(&connection_string).unwrap();

            match conn.run_pending_migrations(MIGRATIONS) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Could not run database migrations: {err}");
                    process::exit(1);
                }
            }
            match option.trim() {
                //"status" => status::display_status(),
                "plan" => plan::interpret(args, &mut conn),
                "subject" => subject::interpret(args, &mut conn),
                "add" => entry::time_setter(&mut conn, args, Mode::ADD),
                "substract" => entry::time_setter(&mut conn, args, Mode::SUBSTRACT),
                "set" => entry::time_setter(&mut conn, args, Mode::SET),
                _ => {}
            }
        }
    }
}

/// It searchs the value of the argument provided.
/// # Arguments
/// * `args` - program arguments
/// * `find` - argument flag to find
pub fn get_specific_arg(args: &mut Vec<String>, find: &str) -> Option<String> {
    match args.iter().enumerate().find(|a| a.1 == find) {
        Some(i) => args.get(i.0 + 1).cloned(),
        None => None,
    }
}

/// Prints the given string and waits for user input. If something different to 'y' is entered, it will end the program
/// with code 0.
/// # Arguments
/// * `warn` - Warn to print before stdin wait.
pub fn request_confirmation(warn: &str) {
    println!("{warn}");
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).expect(
        "Failed to read line. If this keeps ocurring, use --confirm to skip stdin readlines",
    );
    if response.to_lowercase().trim() != "y" && response.to_lowercase().trim() != "yes" {
        println!("Aborting");
        process::exit(0);
    }
}
