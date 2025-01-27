use std::process;
use colored::Colorize;
use diesel::dsl::sql;
use diesel::internal::derives::multiconnection::chrono::Local;
use diesel::internal::operators_macro::FieldAliasMapper;
use diesel::prelude::*;

use diesel::SqliteConnection;
use crate::debug_println;
use crate::error_handler::connection_error_handler;
use crate::models::{DateTime, Period};
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use crate::schema::subjects::dsl::subjects;

fn display_bad_usage() {

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
            connection_error_handler();
            None
        }
    }
}


pub fn interpret(args : &mut Vec<String>) {
    if args.len() < 2 {
        display_bad_usage()
    }
    else {

    }
}