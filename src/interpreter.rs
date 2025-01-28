use std::{env, process};
use colored::Colorize;
use diesel::{Connection, SqliteConnection};
use crate::{debug_println, usage};
use crate::plan::{period, status, subject};

pub fn interpret(args: &mut Vec<String>) {
    match args.len() {
        0 => {
            usage::display_usage();
            process::exit(0);
        },
        _ => {
            let option = args.get(0).unwrap().clone();
            args.remove(0);
            debug_println!("using arg: {option}");
            dotenv::dotenv().ok();
            let mut conn = SqliteConnection::establish(&env::var("DATABASE_URL").unwrap()).unwrap();
            match option.trim() {
                "status" => status::display_status(),
                "plan" => period::interpret(args, &mut conn),
                "subject" => subject::interpret(args),
                _ => {}
            }
        }
    }
}