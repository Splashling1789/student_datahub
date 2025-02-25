use crate::models::Period;
use crate::plan::interpreter::get_plan_arg;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::id;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{delete, RunQueryDsl, SqliteConnection};
use std::process;
use crate::interpreter::request_confirmation;

pub fn remove_plan(conn: &mut SqliteConnection, args: &mut Vec<String>) {
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
}
