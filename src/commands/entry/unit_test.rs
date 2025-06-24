use crate::models::Entry;
use crate::schema::entry::dsl::entry;
use crate::schema::entry::subject_id;
use crate::schema::periods::{description, final_date, initial_date};
use crate::schema::subjects::{name, period_id, short_name};
use crate::schema::*;
use crate::{schema, setup_test_environment, FORMAT};
use assert_cmd::Command;
use diesel::internal::derives::multiconnection::chrono::{Days, Local};
use diesel::ExpressionMethods;
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use diesel::{Connection, QueryDsl};

fn zero_commands(conn: &mut SqliteConnection) {
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]).assert().success();
    assert!(entry
        .filter(subject_id.eq(1))
        .load::<Entry>(conn)
        .unwrap()
        .is_empty());
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]).assert().success();
    assert!(entry
        .filter(subject_id.eq(1))
        .load::<Entry>(conn)
        .unwrap()
        .is_empty());
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]).assert().success();
    assert!(entry
        .filter(subject_id.eq(1))
        .load::<Entry>(conn)
        .unwrap()
        .is_empty());
}

fn add_commands(conn: &mut SqliteConnection) {
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "-1"]).assert().failure();
    assert!(entry
        .filter(subject_id.eq(1))
        .load::<Entry>(conn)
        .unwrap()
        .is_empty());
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "10"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert!(results.len() == 1);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 10);
        assert_eq!(e.date, Local::now().date_naive());
    }
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "0"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert!(results.len() == 1);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 10);
        assert_eq!(e.date, Local::now().date_naive());
    }
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["add", "subj1", "20"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert!(results.len() == 1);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 30);
        assert_eq!(e.date, Local::now().date_naive());
    }
    cmd = Command::cargo_bin("student_datahub").unwrap();
    let tomorrow = Local::now()
        .date_naive()
        .checked_add_days(Days::new(1))
        .unwrap();
    cmd.args(["add", &tomorrow.format(FORMAT).to_string(), "subj1", "20"])
        .assert()
        .success();
    {
        let results = entry
            .filter(subject_id.eq(1))
            .order_by(schema::entry::date)
            .load::<Entry>(conn)
            .unwrap();
        assert!(results.len() == 2);
        let (e1, e2) = (results.get(0).unwrap(), results.get(1).unwrap());
        assert_eq!(e1.dedicated_time, 30);
        assert_eq!(e1.date, Local::now().date_naive());
        assert_eq!(e2.dedicated_time, 20);
        assert_eq!(e2.date, tomorrow);
    }
}

fn substract_commands(conn : &mut SqliteConnection) {
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["substract", "subj1", "10"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert_eq!(results.len(), 2);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 20);
        assert_eq!(e.date, Local::now().date_naive());
    }
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["substract", "subj1", "9999999"]).assert().failure();
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["substract", "subj1", "20"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert_eq!(results.len(), 1);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 20);
        assert_eq!(e.date, Local::now().date_naive().checked_add_days(Days::new(1)).unwrap());
    }
}

fn set_commands(conn : &mut SqliteConnection) {
    let mut cmd = Command::cargo_bin("student_datahub").unwrap();
    let tomorrow = Local::now().date_naive().checked_add_days(Days::new(1)).unwrap().format(FORMAT).to_string();
    cmd.args(["set", &tomorrow, "subj1", "0"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert!(results.is_empty());
    }
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["set", "subj1", "-1"]).assert().failure();
    cmd = Command::cargo_bin("student_datahub").unwrap();
    cmd.args(["set", "subj1", "10"]).assert().success();
    {
        let results = entry.filter(subject_id.eq(1)).load::<Entry>(conn).unwrap();
        assert_eq!(results.len(), 1);
        let e = results.first().unwrap();
        assert_eq!(e.dedicated_time, 10);
        assert_eq!(e.date, Local::now().date_naive());
    }
}

#[test]
fn time_setters_test() {
    use assert_cmd::Command;
    let (_tempdir, mut conn) = setup_test_environment!("student_datahub_time_setter_test");
    let date = Local::now().date_naive();
    insert_into(periods::dsl::periods)
        .values((
            periods::id.eq(1),
            initial_date.eq(date),
            final_date.eq(date.checked_add_days(Days::new(3)).unwrap()),
            description.eq("testing".to_string()),
        ))
        .execute(&mut conn)
        .unwrap();
    insert_into(subjects::dsl::subjects)
        .values((
            subjects::id.eq(1),
            period_id.eq(1),
            short_name.eq("subj1"),
            name.eq("Subject 1"),
        ))
        .execute(&mut conn)
        .unwrap();

    zero_commands(&mut conn);
    add_commands(&mut conn); // 30, 20
    substract_commands(&mut conn); // 0, 20
    set_commands(&mut conn);
}
