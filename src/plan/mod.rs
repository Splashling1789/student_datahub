//! Module for periods (or plans) management.

mod list;
mod modify;
mod remove;
mod start;
mod usage;

use crate::interpreter::{get_specific_arg, request_confirmation};
use crate::models::Period;
use crate::plan::usage::display_bad_usage;
use crate::{debug_println, FORMAT};
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::SqliteConnection;
use std::process;

pub fn get_plan_arg(args: &mut Vec<String>, conn: &mut SqliteConnection) -> i32 {
    match get_specific_arg(args, "--plan") {
        Some(plan_id) => match plan_id.parse::<i32>() {
            Ok(r) => {
                let mut index = None;
                for (i, a) in args.iter_mut().enumerate() {
                    if a.trim() == "--plan" {
                        index = Some(i);
                        break;
                    }
                }
                if let Some(index) = index {
                    args.remove(index);
                    args.remove(index);
                } else {
                    debug_println!("get_plan_arg: no index to remove was found.");
                }
                r
            }
            Err(e) => {
                eprintln!("Failed to parse id.");
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => match Period::get_actual_period(conn) {
            Some(r) => r.id,
            None => {
                eprintln!("No period specified/ocurring now");
                process::exit(1);
            }
        },
    }
}

pub fn get_date_arg(args: &mut Vec<String>, find: &str) -> NaiveDate {
    match get_specific_arg(args, find) {
        Some(start_date) => match NaiveDate::parse_from_str(&start_date, FORMAT) {
            Ok(date) => date,
            Err(e) => {
                eprintln!("Failed to parse date. Remember using format '{}'", FORMAT);
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => {
            usage::display_bad_usage();
            process::exit(1);
        }
    }
}

pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    if args.len() == 0 {
        display_bad_usage();
        process::exit(1);
    } else {
        let option = args.get(0).cloned().unwrap();
        args.remove(0);
        match option.trim() {
            "list" => {
                list::list(conn);
            }
            "start" => {
                if args.len() < 2 {
                    display_bad_usage();
                    process::exit(1);
                } else {
                    let (_start, _end, _description): (NaiveDate, NaiveDate, String) =
                        match args.len() {
                            2 => (
                                Local::now().naive_local().date(),
                                match NaiveDate::parse_from_str(&args[0], FORMAT) {
                                    Ok(date) => date,
                                    Err(e) => {
                                        eprintln!(
                                            "Could not parse date. Remember using format '{}'",
                                            FORMAT
                                        );
                                        eprintln!("{e}");
                                        process::exit(1);
                                    }
                                },
                                args[1].to_string(),
                            ),

                            3 => (
                                match NaiveDate::parse_from_str(&args[0], FORMAT) {
                                    Ok(date) => date,
                                    Err(e) => {
                                        eprintln!(
                                            "Could not parse date. Remember using format '{}'",
                                            FORMAT
                                        );
                                        eprintln!("{e}");
                                        process::exit(1);
                                    }
                                },
                                match NaiveDate::parse_from_str(&args[1], FORMAT) {
                                    Ok(date) => date,
                                    Err(e) => {
                                        eprintln!(
                                            "Could not parse date. Remember using format '{}'",
                                            FORMAT
                                        );
                                        eprintln!("{e}");
                                        process::exit(1);
                                    }
                                },
                                args[2].to_string(),
                            ),
                            _ => {
                                display_bad_usage();
                                process::exit(1);
                            }
                        };
                    start::start_plan(conn, _start, _end, _description);
                }
            }
            "remove" => {
                let plan: i32 = get_plan_arg(args, conn);
                if !args.contains(&"--confirm".to_string()) {
                    let period = match Period::from_id(conn, plan) {
                        Some(period) => period,
                        None => {
                            eprintln!("Plan not found");
                            process::exit(1);
                        }
                    };
                    println!("{}", period.to_string());
                    request_confirmation("Are you sure you want to remove the study plan? [y/n]:");
                }
                remove::remove_plan(conn, plan);
            } // remove command ends here
            "modify" => {
                let plan_id: i32 = get_plan_arg(args, conn);
                let plan = match Period::from_id(conn, plan_id) {
                    Some(period) => period,
                    None => {
                        eprintln!("Failed to fetch period. Does this id exist?");
                        process::exit(1);
                    }
                };
                let new_start_date: NaiveDate = match args.contains(&"--start".to_string()) {
                    true => get_date_arg(args, "--start"),
                    false => plan.initial_date,
                };
                let new_end_date: NaiveDate = match args.contains(&"--end".to_string()) {
                    true => get_date_arg(args, "--end"),
                    false => plan.final_date,
                };
                let descr: String = match args.contains(&"--description".to_string()) {
                    true => match get_specific_arg(args, "--description") {
                        Some(d) => d,
                        None => {
                            display_bad_usage();
                            process::exit(1);
                        }
                    },
                    false => plan.description.clone(),
                };
                modify::modify(conn, plan_id, new_start_date, new_end_date, descr);
            }
            k => {
                debug_println!("No valid argument. Provided: {k}");
                usage::display_bad_usage();
                process::exit(1);
            }
        }
    }
}
