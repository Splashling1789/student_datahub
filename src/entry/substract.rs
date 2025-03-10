use crate::models::{Entry, Subject};
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, dedicated_time, subject_id};
use diesel::dsl::delete;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::SqliteConnection;
use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::process;

fn max(a: i32, b: i32) -> i32 {
    match a < b {
        true => b,
        false => a,
    }
}

pub fn subtract_time(
    conn: &mut SqliteConnection,
    subject: Subject,
    when: NaiveDate,
    amount_to_substract: i32,
) {
    let amount = max(
        Entry::get_time_by_day_and_subject(when, subject.id, conn) - amount_to_substract,
        0,
    );
    if amount == 0 {
        match delete(
            entry
                .filter(subject_id.eq(subject.id))
                .filter(date.eq(when)),
        )
        .execute(conn)
        {
            Ok(_) => {
                println!("Entry added successfully. Current amount: {amount}");
            }
            Err(e) => {
                eprintln!("Failed to delete entry: {e}");
                process::exit(1);
            }
        }
    } else {
        match update(
            entry
                .filter(date.eq(when))
                .filter(subject_id.eq(subject.id)),
        )
        .set(dedicated_time.eq(amount))
        .execute(conn)
        {
            Ok(_) => {
                println!("Entry added successfully. Current amount: {amount}");
            }
            Err(e) => {
                eprintln!("Failed to insert entry: {e}");
                process::exit(1);
            }
        }
    }
}
