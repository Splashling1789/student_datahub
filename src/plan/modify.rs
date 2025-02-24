use diesel::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use std::process;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::{update, SqliteConnection};
use crate::interpreter::get_specific_arg;
use crate::models::Period;
use crate::plan::interpreter::{get_date_arg, get_plan_arg};
use crate::plan::period::get_actual_period;
use crate::plan::usage::display_bad_usage;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, id, initial_date};

pub fn modify(conn: &mut SqliteConnection, args: &mut Vec<String>) {
    let plan_id: i32 = get_plan_arg(args);
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

    for p in crate::plan::period::fetch_all_plans(conn) {
        if p.id != plan_id && p.overlaps((new_start_date, new_end_date)) {
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