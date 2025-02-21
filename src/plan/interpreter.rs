use std::process;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use crate::{debug_println, FORMAT};
use crate::interpreter::get_specific_arg;
use crate::plan::usage::display_bad_usage;

pub fn get_plan_arg(args: &mut Vec<String>) -> i32 {
    match get_specific_arg(args, "--plan")
    {
        Some(plan_id) => match plan_id.parse::<i32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse id.");
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => {
            display_bad_usage();
            process::exit(1);
        }
    }
}

pub fn get_date_arg(args: &mut Vec<String>, find: &str) -> NaiveDate {
    match get_specific_arg(args, find) {
        Some(start_date) => {
            match NaiveDate::parse_from_str(&start_date, FORMAT) {
                Ok(date) => date,
                Err(e) => {
                    eprintln!("Failed to parse date. Remember using format '{}'", FORMAT);
                    debug_println!("{e}");
                    process::exit(1);
                }
            }
        }
        None => {
            display_bad_usage();
            process::exit(1);
        }
    }
}