//! Add time command
use crate::models::Subject;
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, dedicated_time, subject_id};
use diesel::dsl::insert_into;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::{update, ExpressionMethods, QueryDsl};
use diesel::{RunQueryDsl, SqliteConnection};
use std::process;

/// Adds study time to a subject in a specific date
/// # Arguments
/// * `conn` - Database connection
/// * `subject` - Subject studied.
/// * `when` - Date when studied.
/// * `amount_to_add` - Amount to add to the current time.
pub fn add_time(
    conn: &mut SqliteConnection,
    subject: &Subject,
    when: NaiveDate,
    amount_to_add: i32,
) {
    let previous_amount = subject.total_dedicated_time_day(when, conn);
    let amount = previous_amount + amount_to_add;
    // If there was no previous entries, it creates one.
    if previous_amount == 0 && amount > 0 {
        match insert_into(entry)
            .values((
                date.eq(when),
                subject_id.eq(subject.id),
                dedicated_time.eq(amount),
            ))
            .execute(conn)
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert entry: {e}");
                process::exit(1);
            }
        }
    } else if previous_amount > 0 && amount > previous_amount {
        match update(
            entry
                .filter(date.eq(when))
                .filter(subject_id.eq(subject.id)),
        )
        .set(dedicated_time.eq(amount))
        .execute(conn)
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert entry: {e}");
                process::exit(1);
            }
        }
    }
}
