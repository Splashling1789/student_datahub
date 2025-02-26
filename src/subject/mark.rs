use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{final_score, id};
use diesel::ExpressionMethods;
use diesel::{update, QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

pub fn update_mark(conn: &mut SqliteConnection, s: Subject, mark: Option<f32>) {
    match update(subjects.filter(id.eq(s.id)))
        .set(final_score.eq(mark))
        .execute(conn)
    {
        Ok(_) => match mark {
            Some(m) => {
                println!("Successfully marked {} with score {}", s.short_name, m);
            }
            None => {
                println!("Successfully unmarked {}", s.short_name);
            }
        },
        Err(e) => {
            eprintln!("Failed marking: {e}");
            process::exit(1);
        }
    }
}
