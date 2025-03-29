//! # Functionality of add, substract and set commands.
//! This module handles the main data operations for study time registers.
use crate::commands::entry::add::add_time;
use crate::commands::entry::set::set_time;
use crate::commands::entry::substract::subtract_time;
use crate::commands::entry::usage::display_bad_usage;
use crate::commands::subject::get_subject;
use crate::models::Period;
use crate::{format_hours_and_minutes, FORMAT};
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
    Add,
    Substract,
    Set,
}

/// Alters or adds an entry of study time.
/// # Arguments
/// * `conn` - Database connection
/// * `args` - Remaining program arguments
/// * `mode` - Entry altering mode.
pub fn time_setter(conn: &mut SqliteConnection, args: &mut Vec<String>, mode: EntryMode) {
    let when: NaiveDate = match args.len() {
        3 => match NaiveDate::parse_from_str(&args.first().unwrap().clone(), FORMAT) {
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
    let subject = match get_subject(args.first().unwrap(), conn, Some(plan_id)) {
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
        EntryMode::Add => {
            add_time(conn, &subject, when, amount);
        }
        EntryMode::Substract => {
            subtract_time(conn, &subject, when, amount);
        }
        EntryMode::Set => {
            set_time(conn, &subject, when, amount);
        }
    }
    println!(
        "Done! Current dedicated time today: {}",
        format_hours_and_minutes(subject.total_dedicated_time_day(when, conn))
    );
}
