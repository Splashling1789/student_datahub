//! Substract time command
use crate::models::Subject;
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, dedicated_time, subject_id};
use diesel::dsl::delete;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::SqliteConnection;
use diesel::{update, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::process;
use crate::format_hours_and_minutes;

/// Substracts study time to a subject in a specific date
/// # Arguments
/// * `conn` - Database connection
/// * `subject` - Subject studied.
/// * `when` - Date when studied.
/// * `amount_to_substract` - Amount to substract to the current time.
pub fn subtract_time(
    conn: &mut SqliteConnection,
    subject: &Subject,
    when: NaiveDate,
    amount_to_substract: i32,
) {
    let previous_amount = subject.total_dedicated_time_day(when, conn);
    let amount = previous_amount - amount_to_substract;
    if amount < 0 {
        eprintln!("Too much to substract! Remember you've dedicated {}", format_hours_and_minutes(previous_amount));
        process::exit(1);
    }
    if amount == 0 {
        match delete(
            entry
                .filter(subject_id.eq(subject.id))
                .filter(date.eq(when)),
        )
        .execute(conn)
        {
            Ok(_) => {}
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
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to insert entry: {e}");
                process::exit(1);
            }
        }
    }
}
