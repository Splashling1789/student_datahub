//! Manages commands related to subjects.

use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use diesel::{RunQueryDsl, SqliteConnection};
use std::process;

mod add;
pub mod interpreter;
mod list;
mod mark;
mod modify;
mod remove;
mod usage;

/// Fetches all subjects from the database.
/// # Arguments:
/// * conn - Database connection.
fn fetch_all_subjects(conn: &mut SqliteConnection) -> Vec<Subject> {
    match subjects.load::<Subject>(conn) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch subjects: {}", e);
            process::exit(1);
        }
    }
}
