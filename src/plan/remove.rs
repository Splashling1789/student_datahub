use crate::models::Period;
use crate::plan::interpreter::get_plan_arg;
use crate::plan::period::get_actual_period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::id;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{delete, RunQueryDsl, SqliteConnection};
use std::process;

pub fn remove_plan(conn: &mut SqliteConnection, args: &mut Vec<String>) {
    let plan: i32 = get_plan_arg(args);
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
        std::io::stdin().read_line(&mut response).expect(
            "Failed to read line. If this keeps ocurring, use --confirm to skip stdin readlines",
        );
        if response.to_lowercase().trim() != "y" && response.to_lowercase().trim() != "yes" {
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
}
