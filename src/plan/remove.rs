use crate::schema::periods::dsl::periods;
use crate::schema::periods::id;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{delete, RunQueryDsl, SqliteConnection};
use std::process;

pub fn remove_plan(conn: &mut SqliteConnection, plan: i32) {
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
