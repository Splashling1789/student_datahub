//! # Summary of study time.
//! The status command's prompt is divided in three parts:
//! * **Period details**: Prints the date and description of the period.
//! * **Daily summary**: Prints a daily summary, with the total time the user studied, and the time dedicated to every subject.
//! * **Weekly summary**: Prints a weekly summary (if the previous week is included in the plan's period), with how much more did the user study regards the previous week, and whether they are doing better in their average.

mod daily_summary;
mod period_details;
mod weekly_summary;

use crate::commands::plan::get_plan_arg;
use crate::commands::status::daily_summary::daily_summary;
use crate::commands::status::period_details::print_period_details;
use crate::commands::status::weekly_summary::weekly_summary;
use crate::models::{Period, Subject};
use crate::FORMAT;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate, Weekday};
use diesel::SqliteConnection;
use std::process;
use terminal_size::{terminal_size, Width};

/// Day considered the first of the week.
pub const WEEKDAY_START: Weekday = Weekday::Mon;

fn print_separator() {
    let line_longitude = match terminal_size() {
        Some((Width(w), _)) => 3 * w as i32 / 5,
        None => 30,
    };
    for _ in 0..line_longitude {
        print!("-");
    }
    println!();
}

/// Displays the status of the plan, based on program args.
/// # Arguments:
/// * `conn` - Database connection.
/// * `args` - Program arguments.
pub fn display_status(conn: &mut SqliteConnection, args: &mut Vec<String>) {
    let plan_id = get_plan_arg(args, conn);
    let period = Period::from_id(conn, plan_id).unwrap();
    let date = match args.is_empty() {
        true => Local::now().naive_local().date(),
        false => match NaiveDate::parse_from_str(args.first().unwrap(), FORMAT) {
            Ok(date) => date,
            Err(e) => {
                eprintln!("Error parsing date: {e}");
                process::exit(1);
            }
        },
    };
    println!("Current plan: {} (ID:{})", period.description, period.id);
    print_period_details(&period, &date);
    print_separator();
    {
        let mut times: Vec<(Subject, i32)> = Vec::new();
        for i in period.fetch_subjects(conn) {
            let time = i.total_dedicated_time_day(date, conn);
            times.push((i, time));
        }
        let total_time_studied = times.iter().map(|(_, t)| t).sum::<i32>();
        daily_summary(total_time_studied, &times);
    }
    print_separator();
    {
        let week = date.week(WEEKDAY_START);
        let interval = (
            week.first_day().max(period.initial_date),
            week.last_day().min(period.final_date),
        );
        let mut times: Vec<(Subject, i32)> = Vec::new();
        let subject_list = period.fetch_subjects(conn);
        let previous_day = week.first_day().pred_opt().unwrap();
        let total_previous_time: Option<i32>;
        let last_week_final_day: Option<NaiveDate>;
        if previous_day > period.initial_date {
            let previous_week = previous_day.week(WEEKDAY_START);
            let previous_interval = (
                previous_week.first_day().max(period.initial_date),
                previous_week.last_day().min(period.final_date),
            );
            let mut total = 0;
            for i in &subject_list {
                total += i.total_dedicated_time_interval(conn, previous_interval);
            }
            total_previous_time = Some(total);
            last_week_final_day = Some(interval.1);
        } else {
            total_previous_time = None;
            last_week_final_day = None;
        }
        for i in subject_list {
            let time = i.total_dedicated_time_interval(conn, interval);
            times.push((i, time));
        }
        let total_time_studied = times.iter().map(|(_, t)| t).sum::<i32>();
        weekly_summary(
            total_time_studied,
            &times,
            total_previous_time,
            match last_week_final_day {
                Some(d) => Some(period.weekly_average_until(conn, period.initial_date, d)),
                None => None,
            },
        )
    }
}
