use crate::models::Period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{description, final_date, initial_date};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::ExpressionMethods;
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use std::process;

pub fn start_plan(
    conn: &mut SqliteConnection,
    new_start: NaiveDate,
    new_end: NaiveDate,
    new_description: String,
) {
    if new_start.gt(&new_end) {
        eprintln!("Invalid arguments: Start date can't be after end date");
        process::exit(1);
    }

    for p in Period::fetch_all_plans(conn) {
        if p.overlaps((new_start, new_end)) {
            eprintln!("Invalid state: Current study period overlaps the provided period.");
            process::exit(1);
        }
    }

    match insert_into(periods)
        .values((
            initial_date.eq(new_start),
            final_date.eq(new_end),
            description.eq(new_description),
        ))
        .execute(conn)
    {
        Ok(_) => {
            println!("Plan created succesfully");
        }
        Err(e) => {
            println!("Failed to insert: {e}");
            process::exit(1);
        }
    }
}
