///! Handles subject adding.
use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{name, period_id, short_name};
use diesel::dsl::insert_into;
use diesel::ExpressionMethods;
use diesel::{RunQueryDsl, SqliteConnection};
use std::process;

/// Adds a new subject with given data.
/// # Arguments
/// * `conn` - Database connection
/// * `new_plan` - Id of the period of the new subject.
/// * `new_short_name` - Short name of the new subject.
/// * `new_name` - Name of the new subject.
pub fn add(conn: &mut SqliteConnection, new_plan: i32, new_short_name: String, new_name: String) {
    // Two subjects from the same plan can't have the same short name.
    if Subject::fetch_all(conn)
        .iter()
        .any(|s| s.period_id == new_plan && s.short_name.eq(&new_short_name))
    {
        eprintln!("A subject already exists in the period with the same short name.");
        process::exit(1);
    }

    match insert_into(subjects)
        .values((
            short_name.eq(new_short_name),
            name.eq(new_name),
            period_id.eq(new_plan),
        ))
        .execute(conn)
    {
        Ok(_) => {
            println!("Subject added succesfully");
        }
        Err(e) => {
            eprintln!("Could not insert subject into database: {}", e);
            process::exit(1);
        }
    }
}
