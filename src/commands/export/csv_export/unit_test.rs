use std::fs;
use std::ptr::read;
use assert_cmd::assert::OutputAssertExt;
use diesel::{Connection, ExpressionMethods, RunQueryDsl};
use assert_cmd::cargo::CommandCargoExt;
use diesel::dsl::insert_into;
use diesel::internal::derives::multiconnection::chrono::{Days, Local, Months, NaiveDate, TimeDelta};
use rand::Rng;
use crate::commands::export::SqliteConnection;
use crate::commands::export::process::Command;
use crate::schema::*;
use crate::schema::entry::{date, subject_id};
use crate::schema::subjects::{name, period_id, short_name};
use crate::{debug_println, setup_test_environment, FORMAT};

const NUM_SUBJECTS : i32 = 10;

#[test]
fn export_test() {
    let (_tempdir, mut conn) = setup_test_environment!("student_datahub_export_test");
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    let (initial, end) = (Local::now().date_naive(), Local::now().date_naive().checked_add_months(Months::new(3)).unwrap());
    insert_into(periods::dsl::periods)
        .values((periods::id.eq(1), periods::initial_date.eq(initial), periods::final_date.eq(end), periods::description.eq("Testing")))
        .execute(&mut conn).unwrap();
    for i in 1..=NUM_SUBJECTS {
        insert_into(subjects::dsl::subjects)
            .values((subjects::id.eq(i), period_id.eq(1), short_name.eq(&format!("subj{i}")), name.eq(&format!("Subject {i}"))))
            .execute(&mut conn).unwrap();
    }
    let duration_days = (end - initial).num_days() + 1;
    let mut table = vec![vec![0; NUM_SUBJECTS as usize]; duration_days as usize];
    let mut i = initial.clone();
    let mut i_count = 0;
    let mut rnd = rand::rng();
    while i < end {
        for j in 1..=NUM_SUBJECTS {
            if rnd.random_bool(0.2) {
                let amount = rnd.random_range(1..99999);
                table[i_count][(j-1) as usize] = amount;
                insert_into(entry::dsl::entry)
                    .values((entry::date.eq(i), entry::subject_id.eq(j), entry::dedicated_time.eq(amount)))
                    .execute(&mut conn).unwrap();
            }
        }
        i_count += 1;
        i += TimeDelta::days(1);
    }
    cmd.args(["export", "all"]).assert().success();
    let folder = fs::read_dir(_tempdir.path().join(".student_datahub")).ok().unwrap()
        .filter_map(Result::ok)
        .filter(|file| file.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned()).next().unwrap();
    let mut reader_daily = csv::Reader::from_path(_tempdir.path().join(".student_datahub").join(&folder).join("daily.csv")).unwrap();
    i = initial.clone();
    i_count = 0;
    for r in reader_daily.records() {
        let record = r.unwrap();
        debug_println!("{:?} ?= {:?}", record, table);
        let mut read_date = false;
        for (s_count, s) in record.iter().enumerate() {
            if !read_date {
                assert_eq!(NaiveDate::parse_from_str(s, FORMAT).unwrap(), i);
                read_date = true;
            }
            else {
                assert_eq!(table[i_count][s_count-1], s.parse::<i32>().unwrap());
            }
        }
        i += TimeDelta::days(1);
        i_count += 1;
    }

}