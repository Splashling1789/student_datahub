use diesel::internal::derives::multiconnection::chrono::{Local};
use diesel::prelude::*;
use std::process;
use crate::{debug_println};
use crate::models::Period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use diesel::{SqliteConnection};

use crate::plan::list::list;
use crate::plan::modify::modify;
use crate::plan::remove::remove_plan;
use crate::plan::start::start_plan;
use crate::plan::usage::display_bad_usage;

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

pub fn fetch_all_plans(conn: &mut SqliteConnection) -> Vec<Period> {
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
