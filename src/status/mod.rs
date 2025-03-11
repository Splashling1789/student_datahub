//! Displays a summary of study time.

use std::process;
use colored::Colorize;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate, TimeDelta};
use diesel::SqliteConnection;
use terminal_size::{terminal_size, Width};
use crate::{FORMAT};
use crate::models::{Entry, Period, Subject};
use crate::plan::{get_plan_arg};
use crate::schema::periods::initial_date;

pub fn interpret(conn : &mut SqliteConnection, args : &mut Vec<String>) {
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
    if date == period.initial_date {
        print!("{}", date.format(FORMAT).to_string().green())
    }
    else {
        print!("{}", period.initial_date.format(FORMAT).to_string());
    }
    
    if (date - period.initial_date).num_days() == 1 {
        print!(" - {}", date.format(FORMAT).to_string().green());
    }
    else if (date - period.initial_date).num_days() > 1 {
        print!(" - ... - {}", date.format(FORMAT).to_string().green());
    }
        
    if (period.final_date - date).num_days() == 1 {
        print!(" - {}", period.final_date.format(FORMAT).to_string());
    }
    else if (period.final_date - date).num_days() > 1 {
        print!(" - ... - {}", period.final_date.format(FORMAT).to_string());
    }
    println!();
    {
        let line_longitude = match terminal_size() {
            Some((Width(w), _)) => w as i32 / 3,
            None => 30,
        };
        for _ in 0..line_longitude {
            print!("-");
        }
        println!();
    }
    
    if total_time_studied > 0 {
        println!("\tYou have studied a total amount of {total_time_studied} minutes:");
        for i in &times {
            if(i.1 != 0) {
                println!("\t * {} minutes were dedicated on {}", i.1, i.0.name);
            }
        }
    }
    else {
        println!("\tYou haven't studied today yet!");
    }
    
}