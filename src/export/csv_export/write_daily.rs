//! Handles daily csv export format
use crate::export::csv_export::{get_csv_writer, get_header};
use crate::models::Period;
use crate::FORMAT;
use diesel::internal::derives::multiconnection::chrono::{NaiveDate, TimeDelta};
use diesel::SqliteConnection;
use std::ops::Add;
use std::path::PathBuf;
use std::process;

/// Writes the period study time data by days.
/// # Arguments
/// * `conn` - Database connection.
/// * `file` - File path to write.
/// * `period` - Study period.
/// * `date_interval` - Date interval to search entries.
pub(super) fn write_daily(
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
        let mut record: Vec<String> = Vec::new();
        record.push(i.format(FORMAT).to_string());
        for j in &subjects {
            record.push(j.total_dedicated_time_day(i, conn).to_string());
        }
        match writer.write_record(record) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to export data: {e}");
                process::exit(1);
            }
        }
        i = i.add(TimeDelta::days(1));
    }
}
