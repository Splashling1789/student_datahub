use crate::interpreter::get_specific_arg;
use crate::plan::*;
use crate::{debug_println, FORMAT};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::SqliteConnection;
use std::process;

pub fn get_plan_arg(args: &mut Vec<String>) -> i32 {
    match get_specific_arg(args, "--plan") {
        Some(plan_id) => match plan_id.parse::<i32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse id.");
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => {
            usage::display_bad_usage();
            process::exit(1);
        }
    }
}

pub fn get_date_arg(args: &mut Vec<String>, find: &str) -> NaiveDate {
    match get_specific_arg(args, find) {
        Some(start_date) => match NaiveDate::parse_from_str(&start_date, FORMAT) {
            Ok(date) => date,
            Err(e) => {
                eprintln!("Failed to parse date. Remember using format '{}'", FORMAT);
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => {
            usage::display_bad_usage();
            process::exit(1);
        }
    }
}

pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    if args.len() == 0 {
        usage::display_bad_usage();
        process::exit(1);
    } else {
        let option = args.get(0).cloned().unwrap();
        args.remove(0);
        match option.trim() {
            "list" => {
                list::list(conn);
            }
            "start" => {
                start::start_plan(conn, args);
            }
            "remove" => {
                remove::remove_plan(conn, args);
            } // remove command ends here
            "modify" => {
                modify::modify(conn, args);
            }
            k => {
                debug_println!("No valid argument. Provided: {k}");
                usage::display_bad_usage();
                process::exit(1);
            }
        }
    }
}
