use std::ops::Add;
use std::process;
use diesel::SqliteConnection;
use crate::export::usage::display_bad_usage;
use crate::models::Period;
use crate::plan::{get_date_arg, get_plan_arg};

mod usage;
mod csv;

pub fn interpret(args : &mut Vec<String>, conn : &mut SqliteConnection) {
    let plan_id = get_plan_arg(args, conn);
    if args.is_empty() || args.get(0).unwrap().starts_with("--") {
        display_bad_usage();
        process::exit(1);
    }
    let period = match Period::from_id(conn, plan_id) {
        Some(period) => period,
        None => {
            eprintln!("There is no period with the provided id");
            process::exit(1);
        }
    };
    let start_date = match args.contains(&"--start".to_string()) {
        true => Some(get_date_arg(args, "--start")),
        false => None
    };
    let end_date = match args.contains(&"--end".to_string()) {
        true => Some(get_date_arg(args, "--end")),
        false => None
    };
    let mut header = vec![String::from("date")];
    let subjects = period.fetch_subjects(conn);
    for i in &subjects {
        //TODO: Add comma restriction to subjects short name
        let column = i.short_name.clone();
        header.push(column);
    }
    match args.get(0).unwrap().trim() {
        "daily" => {
            
        },
        "weekly" => {
            
        },
        "monthly" => {
            
        },
        "all" => {
            
        },
        _ => {
            display_bad_usage();
            process::exit(1);
        }
    }
}