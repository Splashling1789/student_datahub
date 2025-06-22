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
use crate::{debug_println, FORMAT};
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate, TimeDelta, Weekday};
use diesel::SqliteConnection;
use std::process;
use terminal_size::{terminal_size, Width};
use crate::interpreter::parse_date;

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
        false => parse_date(args.first().unwrap().trim()),
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
        let now_week = date.week(WEEKDAY_START);
        let now_week_interval = (
            now_week.first_day().max(period.initial_date),
            now_week.last_day().min(period.final_date),
        );
        let mut times: Vec<(Subject, i32)> = Vec::new();
        let subject_list = period.fetch_subjects(conn);
        let previous_day = now_week.first_day().pred_opt().unwrap();
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
            last_week_final_day = Some(previous_interval.1);
        } else {
            total_previous_time = None;
            last_week_final_day = None;
        }
        for i in subject_list {
            let time = i.total_dedicated_time_interval(conn, now_week_interval);
            times.push((i, time));
        }
        let total_time_studied = times.iter().map(|(_, t)| t).sum::<i32>();
        debug_println!("last_week: {:?}. Actual date: {date}", last_week_final_day);
        weekly_summary(
            total_time_studied,
            &times,
            total_previous_time,
            match last_week_final_day {
                Some(d) => {
                    debug_println!(
                        "{:?}, {:?}",
                        (d).week(WEEKDAY_START),
                        period.initial_date.week(WEEKDAY_START)
                    );
                    debug_println!(
                        "{:?}, {:?}",
                        (period.initial_date + TimeDelta::weeks(1))
                            .week(WEEKDAY_START)
                            .first_day(),
                        d.week(WEEKDAY_START).first_day()
                    );
                    if ((period.initial_date + TimeDelta::weeks(1)).week(WEEKDAY_START)).first_day()
                        > (d.week(WEEKDAY_START)).first_day()
                    {
                        None
                    } else {
                        Some(period.weekly_average_until(conn, period.initial_date, d))
                    }
                }
                None => None,
            },
        )
    }
}
