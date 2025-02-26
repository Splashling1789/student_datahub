use crate::models::Subject;
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, dedicated_time, subject_id};
use diesel::dsl::delete;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::QueryDsl;
use diesel::{insert_into, update, SqliteConnection};
use diesel::{ExpressionMethods, RunQueryDsl};
use std::process;

pub fn set_time(conn: &mut SqliteConnection, subject: Subject, when: NaiveDate, amount: i32) {
    if amount == 0 {
        match delete(
            entry
                .filter(date.eq(when))
                .filter(subject_id.eq(subject.id)),
        )
        .execute(conn)
        {
            Ok(_) => {
                println!("Entry added successfully. Current amount: {amount}");
            }
            Err(e) => {
                eprintln!("Failed to set entry: {e}");
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
            Ok(r) => {
                if r == 0 {
                    match insert_into(entry)
                        .values((
                            date.eq(when),
                            subject_id.eq(subject.id),
                            dedicated_time.eq(amount),
                        ))
                        .execute(conn)
                    {
                        Ok(_) => {
                            println!("Entry set successfully. Current amount: {amount}");
                        }
                        Err(e) => {
                            eprintln!("Failed to set entry: {e}");
                            process::exit(1);
                        }
                    }
                } else {
                    println!("Entry set successfully. Current amount: {amount}");
                }
            }
            Err(e) => {
                eprintln!("Failed to set entry: {e}");
                process::exit(1);
            }
        }
    }
}
