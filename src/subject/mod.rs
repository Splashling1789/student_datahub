use std::process;
use diesel::{RunQueryDsl, SqliteConnection};
use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;

mod usage;
mod interpreter;
mod add;
mod modify;

fn fetch_all_subjects(conn : &mut SqliteConnection) -> Vec<Subject> {
    match subjects.load::<Subject>(conn) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch subjects: {}", e);
            process::exit(1);
        }
    }
}