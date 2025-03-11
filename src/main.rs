//! # Student Datahub: register your study time.
//! This command-line tool lets you register the time dedicated to all your subjects of different
//! periods (semesters or similar) in a single command. It stores the data in an SQLite database,
//! and there are commands to export it to csv format for later data analysis.
mod entry;
mod export;
mod interpreter;
mod models;
mod plan;
mod schema;
mod subject;
mod usage;

use std::env;
/// Date format for [NaiveDate::parse_from_str][diesel::internal::derives::multiconnection::chrono::NaiveDate::parse_from_str] method
pub const FORMAT: &str = "%m-%d-%Y";

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

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    debug_println!("args: {:?}", args);
    interpreter::interpret(&mut args);
}
