//! # Functionality of add, substract and set commands.
//! This module handles the main data operations for study time registers.
use crate::entry::add::add_time;
use crate::entry::set::set_time;
use crate::entry::substract::subtract_time;
use crate::entry::usage::display_bad_usage;
use crate::models::Period;
use crate::subject::get_subject;
use crate::FORMAT;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::SqliteConnection;
use std::process;

mod add;
mod set;
mod substract;
mod usage;

/// Mode of entry adding
/// * `ADD` - To add time.
/// * `SUBSTRACT` - To substract time.
/// * `SET` - To set time, overriding any previous entry.
pub enum EntryMode {
    ADD,
    SUBSTRACT,
    SET,
}

/// Alters or adds an entry of study time.
/// # Arguments
/// * `conn` - Database connection
/// * `args` - Remaining program arguments
/// * `mode` - Entry altering mode.
pub fn time_setter(conn: &mut SqliteConnection, args: &mut Vec<String>, mode: EntryMode) {
    let when: NaiveDate = match args.len() {
        3 => match NaiveDate::parse_from_str(&*args.get(0).unwrap().clone(), FORMAT) {
            Ok(when) => {
                args.remove(0);
                when
            }
            Err(_) => {
                eprintln!("Error parsing date. Remember using format {FORMAT}");
                process::exit(1);
            }
        },
        _ => Local::now().naive_local().date(),
    };
    if args.len() < 2 {
        display_bad_usage();
        process::exit(1);
    }
    let plan_id = match Period::from_date(conn, &when) {
        Some(plan) => plan.id,
        None => {
            eprintln!("There is no study plan ocurring on the current/specified date.");
            process::exit(1);
        }
    };
    let subject = match get_subject(args.get(0).unwrap(), conn, Some(plan_id)) {
        Some(subject) => subject,
        None => {
            eprintln!("There is no subject with that id or short name");
            process::exit(1);
        }
    };
    let amount = match args.get(1).unwrap().parse::<i32>() {
        Ok(amount) => {
            if amount < 0 {
                eprintln!("The amount of time can't be negative");
                process::exit(1);
            } else {
                amount
            }
        }
        Err(_) => {
            eprintln!("The amount of time must be a positive or zero integer");
            process::exit(1);
        }
    };

    match mode {
        EntryMode::ADD => {
            add_time(conn, subject, when, amount);
        }
        EntryMode::SUBSTRACT => {
            subtract_time(conn, subject, when, amount);
        }
        EntryMode::SET => {
            set_time(conn, subject, when, amount);
        }
    }
}
