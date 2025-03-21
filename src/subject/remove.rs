use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::id;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{delete, RunQueryDsl, SqliteConnection};
use std::process;

/// Removes an existing subject.
/// # Arguments
/// * `conn` - Database connection.
/// * `subj_id` - Subject's id.
pub fn remove(conn: &mut SqliteConnection, subj_id: i32) {
    match delete(subjects.filter(id.eq(subj_id))).execute(conn) {
        Ok(_) => {
            println!("Subject removed succesfully");
        }
        Err(e) => {
            eprintln!("Error deleting subject: {e}");
            process::exit(1);
        }
    }
}
