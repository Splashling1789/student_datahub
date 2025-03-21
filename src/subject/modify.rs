use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{id, name, short_name};
use diesel::ExpressionMethods;
use diesel::{update, QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

/// Modifies an existing subject
/// # Arguments
/// * `conn` - Database connection.
/// * `subj` - Subject to modify.
/// * `new_short_name` - New subject's short name
/// * `new_name` - New subject's name.
pub fn modify(
    conn: &mut SqliteConnection,
    subj: Subject,
    new_short_name: String,
    new_name: String,
) {
    // Two subjects from the same plan can't have the same short name.
    if Subject::fetch_all(conn).iter().any(|s| {
        s.id != subj.id && s.period_id == subj.period_id && s.short_name.eq(&new_short_name)
    }) {
        eprintln!("A subject already exists in the period with the same short name.");
        process::exit(1);
    }
    match update(subjects.filter(id.eq(subj.id)))
        .set((short_name.eq(new_short_name), name.eq(new_name)))
        .execute(conn)
    {
        Ok(_) => {
            println!("Subjects edited succesfully.");
        }
        Err(e) => {
            eprintln!("Failed to update the subject: {}", e);
            process::exit(1);
        }
    }
}
