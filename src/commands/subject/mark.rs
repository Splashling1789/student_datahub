//! Handles subject marking/scoring.

use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{final_score, id};
use diesel::ExpressionMethods;
use diesel::{update, QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

/// It updates a subject's mark.
/// # Arguments
/// * `conn` - Database connection.
/// * `subject` - Subject to modify its mark.
/// * `mark` - New mark, where None means the subject has no mark.
pub fn update_mark(conn: &mut SqliteConnection, subject: Subject, mark: Option<f32>) {
    match update(subjects.filter(id.eq(subject.id)))
        .set(final_score.eq(mark))
        .execute(conn)
    {
        Ok(_) => match mark {
            Some(m) => {
                println!(
                    "Successfully marked {} with score {}",
                    subject.short_name, m
                );
            }
            None => {
                println!("Successfully unmarked {}", subject.short_name);
            }
        },
        Err(e) => {
            eprintln!("Failed marking: {e}");
            process::exit(1);
        }
    }
}
