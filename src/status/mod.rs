//! Displays a summary of study time.

mod period_details;
mod daily_summary;

use std::process;
use colored::Colorize;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::SqliteConnection;
use crate::{FORMAT};
use crate::models::{Entry, Period, Subject};
use crate::plan::{get_plan_arg};
use crate::status::daily_summary::daily_summary;
use crate::status::period_details::print_period_details;

pub fn display_status(conn : &mut SqliteConnection, args : &mut Vec<String>) {
    let plan_id = get_plan_arg(args, conn);
    let period = Period::from_id(conn, plan_id).unwrap();
    let date = match args.is_empty() {
        true => Local::now().naive_local().date(),
        false => match NaiveDate::parse_from_str(args.get(0).unwrap(), FORMAT) {
            Ok(date) => date,
            Err(e) => {
                eprintln!("Error parsing date: {e}");
                process::exit(1);
            },
        }
    };
    let mut times : Vec<(Subject, i32)> = Vec::new();
    for i in period.fetch_subjects(conn) {
        let time = Entry::get_time_by_day_and_subject(date, i.id, conn);
        times.push((i, time));
    }

    let total_time_studied = times.iter().map(|(_, t)| t).sum::<i32>();
    println!("Current plan: {} (ID:{})", period.description, period.id);
    print_period_details(&period, &date);
    daily_summary(total_time_studied, &times);
    
    
}