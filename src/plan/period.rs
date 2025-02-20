use colored::Colorize;
use diesel::dsl::insert_into;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::prelude::*;
use std::process;

use crate::debug_println;
use crate::models::Period;
use crate::plan::period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, id, initial_date};
use diesel::{delete, update, SqliteConnection};

pub const FORMAT: &str = "%m-%d-%Y";

pub fn display_bad_usage() {
    println!(
        "Bad usage: {} plan ...:\n
        - start [start] (end) (description) : Starts a new study plan. It starts today if no start date is provided.
        - list : Lists all the study periods.
        - modify [--plan (plan id)] [--start (new start date)] [--end (new end date)] [--description (new description)] : Modifies the current plan (or one determined by an id).
        - remove [--plan (plan id)] [--confirm] : Removes the actual study plan (or one determined by id). Use the --confirm option to do so without any warning.
        The date format is: {FORMAT}\n\
    ", crate::env::args().collect::<Vec<String>>().first().unwrap());
}

pub fn get_actual_period(conn: &mut SqliteConnection) -> Option<Period> {
    let now = Local::now().date_naive();
    match periods
        .filter(initial_date.le(now))
        .filter(final_date.ge(now))
        .load::<Period>(conn)
    {
        Ok(period) => {
            if period.len() > 1 {
                debug_println!(
                    "There is more than one period ocurring now! Content: {:?}",
                    period
                );
            }
            period.first().cloned()
        }
        Err(e) => {
            eprintln!("Failed to load: {e}");
            process::exit(1);
        }
    }
}

fn get_plan_arg(args: &mut Vec<String>) -> i32 {
    match get_specific_arg(args, "--plan")
    {
        Some(plan_id) => match plan_id.parse::<i32>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse id.");
                debug_println!("{e}");
                process::exit(1);
            }
        },
        None => {
            period::display_bad_usage();
            process::exit(1);
        }
    }
}

fn get_date_arg(args: &mut Vec<String>, find: &str) -> NaiveDate {
    match get_specific_arg(args, find) {
        Some(start_date) => {
            match NaiveDate::parse_from_str(&start_date, FORMAT) {
                Ok(date) => date,
                Err(e) => {
                    eprintln!("Failed to parse date. Remember using format '{}'", FORMAT);
                    debug_println!("{e}");
                    process::exit(1);
                }
            }
        }
        None => {
            display_bad_usage();
            process::exit(1);
        }
    }
}

fn get_specific_arg(args: &mut Vec<String>, find : &str) -> Option<String> {
    args
        .get(args.iter().enumerate().find(|a| a.1 == find).unwrap().0 + 1)
        .cloned()
}

fn fetch_all_plans(conn: &mut SqliteConnection) -> Vec<Period> {
    match periods.load::<Period>(conn) {
        Ok(p) => {
            p
        }
        Err(e) => {
            eprintln!("Failed to load the periods.");
            debug_println!("{e}");
            process::exit(1);
        }
    }
}

fn is_actual(p: &Period) -> bool {
    let now = Local::now().date_naive();
    if now >= p.initial_date && now <= p.final_date {
        true
    } else {
        false
    }
}

fn period_overlaps(p1 : (NaiveDate, NaiveDate), p2 : (NaiveDate, NaiveDate)) -> bool {
    (p1.0 <= p2.1 && p1.0 >= p2.0)
        || (p1.1 <= p2.1 && p1.1 >= p2.0)
        || (p1.0 <= p2.0 && p1.1 >= p2.0)

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
                        println!("{}", i.to_string().green());
                    } else {
                        println!("{}", i.to_string());
                    }
                }
            } // list command ends here
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

                    if _end < _start {
                        eprintln!("Invalid arguments: End date can't be before start date.");
                        process::exit(1);
                    }

                    if let Some(period) = get_actual_period(conn) {
                        if period_overlaps((period.initial_date, period.final_date), (_start, _end))
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
            } // start command ends here
            "remove" => {
                let plan: i32 = match args.contains(&"--plan".to_string()) {
                    true => get_plan_arg(args),
                    false => {
                        match get_actual_period(conn) {
                            Some(p) => p.id,
                            None => {
                                eprintln!("Invalid state: There is no actual period. You may want to specify it using --plan argument");
                                process::exit(1);
                            }
                        }
                    }
                };
                if !args.contains(&"--confirm".to_string()) {
                    let period = match periods.filter(id.eq(plan)).load::<Period>(conn) {
                        Ok(period) => match period.first().cloned() {
                            Some(period) => period,
                            None => {
                                eprintln!("Plan not found");
                                process::exit(1);
                            }
                        },
                        Err(e) => {
                            println!("Failed to fetch period: {e}");
                            process::exit(1);
                        }
                    };
                    println!("{}", period.to_string());
                    println!("Are you sure you want to remove the study plan? [y/n]: ");
                    let mut response = String::new();
                    std::io::stdin().read_line(&mut response)
                        .expect("Failed to read line. If this keeps ocurring, use --confirm to skip stdin readlines");
                    if response.to_lowercase().trim() != "y"
                        && response.to_lowercase().trim() != "yes"
                    {
                        println!("Aborting");
                        process::exit(0);
                    }
                }

                match delete(periods.filter(id.eq(plan))).execute(conn) {
                    Ok(_) => {
                        println!("Plan deleted successfully");
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Failed to delete: {e}");
                        process::exit(1);
                    }
                }
            } // remove command ends here
            "modify" => {
                let plan_id: i32 = match args.contains(& "--plan".to_string()) {
                    true => {
                        get_plan_arg(args)
                    }
                    false => {
                        match get_actual_period(conn) {
                            Some(period) => {
                                period.id
                            }
                            None => {
                                eprintln!("No plan id was provided and there is no actual plan");
                                process::exit(1);
                            }
                        }
                    }
                };
                let plan = match periods.filter(id.eq(plan_id)).load::<Period>(conn) {
                    Ok(period) => match period.first().cloned() {
                        Some(period) => period,
                        None => {
                            eprintln!("Failed to fetch period. Does this id exist?");
                            process::exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to fetch period: {e}");
                        process::exit(1);
                    }
                };
                let new_start_date : NaiveDate = match args.contains(& "--start".to_string()) {
                    true => {
                        get_date_arg(args, "--start")
                    }
                    false => {
                        plan.initial_date
                    }
                };

                let new_end_date : NaiveDate = match args.contains(& "--end".to_string()) {
                    true => {
                        get_date_arg(args, "--end")
                    }
                    false => {
                        plan.final_date
                    }
                };

                for p in fetch_all_plans(conn) {
                    if period_overlaps((new_start_date, new_end_date), (p.initial_date, p.final_date)) {
                        eprintln!("The modified period cannot overlap another period.");
                        eprintln!("Overlapped period: {}", p.to_string());
                        process::exit(1);
                    }
                }

                let descr : String = match args.contains(& "--description".to_string()) {
                    true => {
                        match get_specific_arg(args, "--description") {
                            Some(d) => d,
                            None => {
                                display_bad_usage();
                                process::exit(1);
                            }
                        }
                    }
                    false => {
                        plan.description.clone()
                    }
                };

                match update(periods.filter(id.eq(plan_id))).set((initial_date.eq(new_start_date), final_date.eq(new_end_date), description.eq(descr))).execute(conn) {
                    Ok(_) => {
                        println!("The plan modified succesfully");
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Failed to update period: {e}");
                        process::exit(1);
                    }
                }

            }
            k => {
                debug_println!("No valid argument. Provided: {k}");
                display_bad_usage();
                process::exit(1);
            }
        }
    }
}
