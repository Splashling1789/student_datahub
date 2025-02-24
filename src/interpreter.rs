//! Main interpreter
//! This module interprets the arguments given by the user and delegates
//! the work to command submodules ([plan], [subject], [export]). It also provides
//! useful functions to every command submodule.

use crate::{debug_println, plan, subject, usage};
use diesel::{Connection, SqliteConnection};
use std::{env, process};

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
            let mut conn = SqliteConnection::establish(&env::var("DATABASE_URL").unwrap()).unwrap();
            match option.trim() {
                //"status" => status::display_status(),
                "plan" => plan::interpreter::interpret(args, &mut conn),
                "subject" => subject::interpreter::interpret(args, &mut conn),
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
