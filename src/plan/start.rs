use diesel::ExpressionMethods;
use std::process;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use crate::FORMAT;
use crate::plan::period::get_actual_period;
use crate::plan::usage::display_bad_usage;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, initial_date};

pub fn start_plan(conn : &mut SqliteConnection, args: &mut Vec<String>) {
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

        if _end < _start {
            eprintln!("Invalid arguments: End date can't be before start date.");
            process::exit(1);
        }

        if let Some(period) = get_actual_period(conn) {
            if period.overlaps((_start, _end))
            {
                eprintln!(
                    "Invalid state: Current study period overlaps the provided period."
                );
                process::exit(1);
            }
        }

        match insert_into(periods)
            .values((
                initial_date.eq(_start),
                final_date.eq(_end),
                description.eq(_description),
            ))
            .execute(conn)
        {
            Ok(_) => {
                println!("The plan created succesfully");
            }
            Err(e) => {
                println!("Failed to insert: {e}");
                process::exit(1);
            }
        }
    }
}