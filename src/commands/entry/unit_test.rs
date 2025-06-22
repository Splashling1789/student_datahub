#[cfg(test)]
use crate::{debug_println, fs};
use diesel::{Connection, QueryDsl};
use diesel::ExpressionMethods;
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use diesel::internal::derives::multiconnection::chrono::{Days, Local};
use crate::{setup_test_environment, FORMAT};
use crate::models::{Entry, Period};
use crate::schema::*;
use crate::schema::entry::dsl::entry;
use crate::schema::entry::subject_id;
use crate::schema::periods::{description, final_date, initial_date};
use crate::schema::subjects::{name, period_id, short_name};

#[test]
fn time_setters_test() {
    use assert_cmd::Command;
    let (_TEMPDIR, mut conn) = setup_test_environment!();
    let date = Local::now().date_naive();
    insert_into(periods::dsl::periods)
        .values((periods::id.eq(1),
                 initial_date.eq(date),
                 final_date.eq(date.checked_add_days(Days::new(3)).unwrap()),
                 description.eq(format!("testing")))).execute(&mut conn).unwrap();

    insert_into(subjects::dsl::subjects)
        .values((subjects::id.eq(1),
                 period_id.eq(1),
                 short_name.eq("subj1"),
                 name.eq("Subject 1"))).execute(&mut conn).unwrap();
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]);
    assert!(
        entry.filter(subject_id.eq(1))
            .load::<Entry>(&mut conn)
            .unwrap().is_empty()
    );
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]);
    assert!(
        entry.filter(subject_id.eq(1))
            .load::<Entry>(&mut conn)
            .unwrap().is_empty()
    );
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]);
    assert!(
        entry.filter(subject_id.eq(1))
            .load::<Entry>(&mut conn)
            .unwrap().is_empty()
    );
}