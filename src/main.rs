//! # Student Datahub: register your study time.
//! This command-line tool lets you register the time dedicated to all your subjects of different
//! periods (semesters or similar) in a single command. It stores the data in an SQLite database,
//! and there are commands to export it to csv format for later data analysis.
mod interpreter;
mod models;
mod schema;
mod usage;
mod db_connection_handler;
mod commands;

use std::{env, fs};
use std::path::Path;

/// Date format for [NaiveDate::parse_from_str][diesel::internal::derives::multiconnection::chrono::NaiveDate::parse_from_str] method
pub const FORMAT: &str = "%d-%m-%Y";
/// It prints a formatted message (just like println! would), with '\[DEBUG]' prefix and colored in yellow.
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            use colored::Colorize;
            println!("{}", format!("[DEBUG] {}", format!($($arg)*)).yellow());
        }
    };
}

/// It builds a formatted string based on the given time (in minutes). 
/// # Arguments
/// * `time` - Time to format, in minutes.
pub fn format_hours_and_minutes(time : i32) -> String {
    if time / 60 == 0 {
        format!("{}min", time)
    }
    else {format!("{}h {}min", time / 60, time % 60)}
}

/// It gets a String with the path of the program data folder. If it doesn't exist, then it's created.
/// Note that depending on the OS the data dir will be different.
/// * If the OS is Windows, the folder will be `%APPDATA%\.student_datahub`
/// * Else (assumming it will be a UNIX-like OS like Linux or macOS), the folder will be `%HOME%/.student_datahub`
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

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    debug_println!("args: {:?}", args);
    interpreter::interpret(&mut args);
}
