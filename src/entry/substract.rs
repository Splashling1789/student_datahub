use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::process;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::{insert_into, SqliteConnection};
use diesel::dsl::delete;
use crate::FORMAT;
use crate::models::{Entry, Period};
use crate::schema::entry::{date, dedicated_time, subject_id};
use crate::schema::entry::dsl::entry;
use crate::subject::interpreter::get_subject;

fn max(a : i32, b : i32) -> i32 {
    match a < b {
        true => b,
        false => a,
    }
}

pub fn subtract_time(conn : &mut SqliteConnection, args : &mut Vec<String>) {
    let when: NaiveDate = match args.len() {
        3 => match NaiveDate::parse_from_str(&*args.get(0).unwrap().clone(), FORMAT) {
            Ok(when) => {
                args.remove(0);
                when
            },
            Err(e) => {
                eprintln!("Error parsing date. Remember using format {FORMAT}");
                process::exit(1);
            },
        },
        _ => Local::now().naive_local().date()
    };
    let plan_id = match Period::get_period_from_date(conn, &when) {
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
    let amount_to_add = match args.get(1).unwrap().parse::<i32>() {
        Ok(amount) => {
            if amount <= 0 {
                eprintln!("The amount of time must be a positive integer");
                process::exit(1);
            }
            else {
                amount
            }
        },
        Err(e) => {
            eprintln!("The amount of time must be a positive integer");
            process::exit(1);
        }
    };
    let amount = max(Entry::get_time_by_day_and_subject(when, subject.id, conn) - amount_to_add, 0);
    if amount == 0 {
        match delete(entry.filter(subject_id.eq(subject.id)).filter(date.eq(when))).execute(conn) {
            Ok(_) => {
                println!("Entry added successfully. Current amount: {amount}");
            }
            Err(e) => {
                eprintln!("Failed to delete entry: {e}");
                process::exit(1);
            }
        }
    }
    else {
        match insert_into(entry).values((date.eq(when), subject_id.eq(subject.id), dedicated_time.eq(amount))).execute(conn) {
            Ok(_) => {
                println!("Entry added successfully. Current amount: {amount}");
            }
            Err(e) => {
                eprintln!("Failed to insert entry: {e}");
                process::exit(1);
            }
        }
    }
}