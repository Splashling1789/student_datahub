//! Displays a summary of study time.

mod daily_summary;
mod period_details;
mod weekly_summary;

use crate::models::{Period, Subject};
use crate::plan::get_plan_arg;
use crate::status::daily_summary::daily_summary;
use crate::status::period_details::print_period_details;
use crate::status::weekly_summary::weekly_summary;
use crate::FORMAT;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate, TimeDelta, Weekday};
use diesel::SqliteConnection;
use std::ops::Add;
use std::process;
use terminal_size::{terminal_size, Width};

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

pub fn display_status(conn: &mut SqliteConnection, args: &mut Vec<String>) {
    let plan_id = get_plan_arg(args, conn);
    let period = Period::from_id(conn, plan_id).unwrap();
    let date = match args.is_empty() {
        true => Local::now().naive_local().date(),
        false => match NaiveDate::parse_from_str(args.get(0).unwrap(), FORMAT) {
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
                Some(d) => Some(weekly_average_until(conn, d, period)),
                None => None,
            },
        )
    }
}

fn weekly_average_until(conn: &mut SqliteConnection, day: NaiveDate, period: Period) -> f32 {
    let mut i = period.initial_date;
    let mut count = 0;
    let mut sum = 0;
    while i.week(WEEKDAY_START).last_day() < day {
        for j in period.fetch_subjects(conn) {
            sum += j.total_dedicated_time_week(conn, i.week(WEEKDAY_START));
        }
        count += 1;
        i = i.add(TimeDelta::weeks(1));
    }
    (sum as f32) / (count as f32)
}
