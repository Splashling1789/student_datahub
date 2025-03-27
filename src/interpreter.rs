//! # Main interpreter
//! This module interprets the arguments given by the user and delegates
//! the work to command submodules ([plan], [subject], [export]). It also provides
//! useful functions to every command submodule.

use crate::entry::EntryMode;
use crate::{debug_println, entry, export, plan, status, subject, usage};
use std::{process};
use crate::db_connection_handler::stablish_and_run_migrations;

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
            let mut conn = stablish_and_run_migrations();
            match option.trim() {
                "status" => status::display_status(&mut conn, args),
                "plan" => plan::interpret(args, &mut conn),
                "subject" => subject::interpret(args, &mut conn),
                "add" => entry::time_setter(&mut conn, args, EntryMode::ADD),
                "substract" => entry::time_setter(&mut conn, args, EntryMode::SUBSTRACT),
                "set" => entry::time_setter(&mut conn, args, EntryMode::SET),
                "export" => export::interpret(args, &mut conn),
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

/// It searches for arguments that matches a prefix and are not in a given vector, and returns (if any) the first found.
/// This is used for some commands that require options starting with '--', so that the user doesn't input wrong options.
/// # Arguments
/// * `args` - Arguments.
/// * `options` - Arguments that should be in the arguments.
/// * `prefix` - Prefix to select arguments.
pub fn detect_unknown_arg(args : &Vec<String>, options : &Vec<&str>, prefix: &str) -> Option<String> {
    for i in args {
        if i.starts_with(prefix) {
            let mut found = false;
            for j in options {
                if i.eq(j) {
                    found = true;
                    break;
                }
            }
            if !found {
                return Some(i.to_string());
            }
        }
    }
    None
}