use std::process;
use colored::Colorize;
use diesel::dsl::{insert_into};
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::prelude::*;

use diesel::SqliteConnection;
use crate::debug_println;
use crate::models::{Period};
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, initial_date};

const FORMAT : &str = "%m-%d-%Y";

fn display_bad_usage() {
    println!(
        "Bad usage: {} plan ...:\n|
        - start [start] (end) (description) : Starts a new study plan. It starts today if no start date is provided\n\
        - modify [--start (new start date)] [--end (new end date)] [--description (new description)] : Modifies the current plan\n\
        - remove [--confirm] : Removes the actual study plan. Use the --confirm option to do so without any warning.\n\
        The date format is: {FORMAT}\n\
    ", crate::env::args().collect::<Vec<String>>().first().unwrap());
}

pub fn get_actual_period(conn: &mut SqliteConnection) -> Option<Period> {
    let now = Local::now().date_naive();
    match periods.filter(initial_date.le(now)).filter(final_date.ge(now)).load::<Period>(conn) {
        Ok(period) => {
            if period.len() != 1 {
                debug_println!("There is more than one period ocurring now! Content: {:?}", period);
            }
            period.first().cloned()
        }
        Err(e) => {
            eprintln!("Failed to load: {e}");
            process::exit(1);
        }
    }
}

fn is_actual(p : &Period) -> bool{
    let now = Local::now().date_naive();
    if now >= p.initial_date && now <= p.final_date {true}
    else {false}
}

pub fn interpret(args : &mut Vec<String>, conn : &mut SqliteConnection) {
    if args.len() < 2 {
        display_bad_usage()
    }
    else {
        let option = args.get(0).cloned().unwrap();
        args.remove(0);
        match option.trim() {
            "list" => {
                let list = match periods.order_by(initial_date).load::<Period>(conn) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Failed to fetch: {e}");
                        process::exit(1);
                    }
                };
                println!("Study periods:");
                if list.is_empty() {
                    println!("No periods created yet.");
                }
                for i in list {
                    if is_actual(&i) {
                        println!("{}", format!("{}-{}\t{}", i.initial_date.format(FORMAT).to_string(), i.final_date.format(FORMAT).to_string(), i.description.to_string()).green());
                    }
                    else {
                        println!("{}-{}\t{}", i.initial_date.format(FORMAT).to_string(), i.final_date.format(FORMAT).to_string(), i.description.to_string());
                    }

                }
            }
            "start" => {
                if args.len() < 2 {
                    display_bad_usage();
                    process::exit(1);
                }
                else {
                    let (_start, _end, _description) : (NaiveDate, NaiveDate, String) = match args.len() {
                        2 => (
                            Local::now().naive_local().date(),
                            match NaiveDate::parse_from_str(&args[0], FORMAT){
                                Ok(date) => date,
                                Err(e) => {
                                    eprintln!("Could not parse date. Remember using format '{}'", FORMAT);
                                    eprintln!("{e}");
                                    process::exit(1);
                                }
                            },
                            args[1].to_string()),

                        3 => (
                            match NaiveDate::parse_from_str(&args[0], FORMAT) {
                                Ok(date) => date,
                                Err(e) => {
                                    eprintln!("Could not parse date. Remember using format '{}'", FORMAT);
                                    eprintln!("{e}");
                                    process::exit(1);
                                }
                            },
                            match NaiveDate::parse_from_str(&args[1], FORMAT) {
                                Ok(date) => date,
                                Err(e) => {
                                    eprintln!("Could not parse date. Remember using format '{}'", FORMAT);
                                    eprintln!("{e}");
                                    process::exit(1);
                                }
                            },
                            args[2].to_string()),
                        _ => {display_bad_usage(); process::exit(1);}
                    };

                    if _end < _start {
                        eprintln!("Invalid arguments: End date can't be before start date.");
                        process::exit(1);
                    }

                    if let Some(period) = get_actual_period(conn) {
                        if (period.initial_date <= _end && period.initial_date >= _start) ||
                            (period.final_date <= _end && period.final_date >= _start) ||
                            (period.initial_date <= _start && period.final_date >= _end) {
                            eprintln!("Invalid arguments: Current study period overlaps the provided period.");
                            process::exit(1);
                        }
                    }

                    match insert_into(periods)
                        .values((initial_date.eq(_start), final_date.eq(_end), description.eq(_description)))
                        .execute(conn) {
                        Ok(_) => {
                            println!("The plan created succesfully");
                        }
                        Err(e) => {
                            println!("Failed to insert: {e}");
                            process::exit(1);
                        }
                    }
                }
            } // start command ends here
            _ => {
                display_bad_usage();
                process::exit(1);
            }
        }
    }
}