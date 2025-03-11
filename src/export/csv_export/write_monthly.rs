use std::path::PathBuf;
use std::process;
use diesel::internal::derives::multiconnection::chrono::{Datelike, NaiveDate};
use diesel::SqliteConnection;
use crate::export::csv_export::{get_csv_writer, get_header, MONTHLY_FORMAT};
use crate::models::{Entry, Period};

pub(super) fn write_monthly(conn : &mut SqliteConnection, file : &PathBuf, period: &Period, date_interval : (&NaiveDate, &NaiveDate)) {
    let subjects = period.fetch_subjects(conn);
    let mut writer = get_csv_writer(file);
    match  writer.write_record(get_header(&subjects)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to write when writing header: {e}");
            process::exit(1);
        }
    }
    let mut i  = date_interval.0.clone();
    while i.le(&date_interval.1) {
        let interval_to_fetch = (
            NaiveDate::from_ymd_opt(i.year(), i.month(), 1).unwrap(),
            NaiveDate::from_ymd_opt(i.year(), i.month() + 1, 1).unwrap().pred_opt().unwrap(),
        );
        let mut record : Vec<String> = Vec::new();
        record.push(format!("{}", i.format(MONTHLY_FORMAT)));
        for j in &subjects {
            record.push(Entry::get_time_by_interval_and_subject(conn, (Some(interval_to_fetch.0), Some(interval_to_fetch.1)), j.id).to_string());
        }
        match writer.write_record(record) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to export data: {e}");
                process::exit(1);
            }
        }
        i = i.with_month(std::cmp::max((i.month() + 1) % 13 , 1)).unwrap();
    }
}