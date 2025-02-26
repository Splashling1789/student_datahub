//! Module for modifying existing periods.

use crate::models::Period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, id, initial_date};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::ExpressionMethods;
use diesel::{update, SqliteConnection};
use diesel::{QueryDsl, RunQueryDsl};
use std::process;

pub fn modify(
    conn: &mut SqliteConnection,
    plan_id: i32,
    new_start_date: NaiveDate,
    new_end_date: NaiveDate,
    new_description: String,
) {
    for p in Period::fetch_all_plans(conn) {
        if p.id != plan_id && p.overlaps((new_start_date, new_end_date)) {
            eprintln!("The modified period cannot overlap another period.");
            eprintln!("Overlapped period: {}", p.to_string());
            process::exit(1);
        }
    }

    match update(periods.filter(id.eq(plan_id)))
        .set((
            initial_date.eq(new_start_date),
            final_date.eq(new_end_date),
            description.eq(new_description),
        ))
        .execute(conn)
    {
        Ok(_) => {
            println!("The plan modified succesfully");
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to modify period: {e}");
            process::exit(1);
        }
    }
}
