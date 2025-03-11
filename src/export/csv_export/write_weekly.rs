use super::{get_csv_writer, get_header, WEEKDAY_START};
use crate::models::{Entry, Period};
use crate::FORMAT;
use diesel::internal::derives::multiconnection::chrono::{NaiveDate, TimeDelta};
use diesel::SqliteConnection;
use std::ops::Add;
use std::path::PathBuf;
use std::process;
/// Writes the period study time data by weeks.
/// # Arguments
/// * `conn` - Database connection.
/// * `file` - File path to write.
/// * `period` - Study period.
/// * `date_interval` - Date interval to search entries.
pub(super) fn write_weekly(
    conn: &mut SqliteConnection,
    file: &PathBuf,
    period: &Period,
    date_interval: (&NaiveDate, &NaiveDate),
) {
    let subjects = period.fetch_subjects(conn);
    let mut writer = get_csv_writer(file);
    match writer.write_record(get_header(&subjects)) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write when writing header: {e}");
            process::exit(1);
        }
    }
    let mut i = date_interval.0.clone();
    while i.le(&date_interval.1) {
        let interval_to_fetch = (
            i.week(WEEKDAY_START)
                .first_day()
                .max(date_interval.0.clone()),
            i.week(WEEKDAY_START)
                .last_day()
                .min(date_interval.1.clone()),
        );
        let mut record: Vec<String> = Vec::new();
        record.push(format!(
            "{}:{}",
            interval_to_fetch.0.format(FORMAT),
            interval_to_fetch.1.format(FORMAT)
        ));
        for j in &subjects {
            record.push(
                Entry::get_time_by_interval_and_subject(
                    conn,
                    (Some(interval_to_fetch.0), Some(interval_to_fetch.1)),
                    j.id,
                )
                .to_string(),
            );
        }
        match writer.write_record(record) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to export data: {e}");
                process::exit(1);
            }
        }
        i = i.add(TimeDelta::weeks(1));
    }
}
