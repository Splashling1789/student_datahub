use crate::plan::{period};
use crate::{debug_println, usage};
use diesel::{Connection, SqliteConnection};
use std::{env, process};

pub fn interpret(args: &mut Vec<String>) {
    match args.len() {
        0 => {
            usage::display_usage();
            process::exit(0);
        }
        _ => {
            let option = args.get(0).unwrap().clone();
            args.remove(0);
            debug_println!("using arg: {option}");
            dotenv::dotenv().ok();
            let mut conn = SqliteConnection::establish(&env::var("DATABASE_URL").unwrap()).unwrap();
            match option.trim() {
                //"status" => status::display_status(),
                "plan" => period::interpret(args, &mut conn),
                //"subject" => subject::interpret(args),
                _ => {}
            }
        }
    }
}
pub fn get_specific_arg(args: &mut Vec<String>, find : &str) -> Option<String> {
    args
        .get(args.iter().enumerate().find(|a| a.1 == find).unwrap().0 + 1)
        .cloned()
}

