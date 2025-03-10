use std::collections::LinkedList;
use std::fmt::Display;
use std::ops::Add;
use std::path::PathBuf;
use std::process;
use diesel::internal::derives::multiconnection::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeDelta};
use diesel::SqliteConnection;
use super::ExportMode;
use crate::FORMAT;
use crate::models::{Entry, Period, Subject};

const DATETIME_EXPORT_FORMAT : &str = "%Y%m%d_%H%M%S";


fn get_header(subjects : &Vec<Subject>) -> Vec<String> {
    let mut header = vec![String::from("date")];
    for i in subjects {
        let column = i.short_name.clone();
        header.push(column);
    }
    header
}

fn get_file_path(path: &str, mode : &ExportMode, descr : String) -> PathBuf {
    match mode {
        ExportMode::DAILY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::WEEKLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}weekly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\weekly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::MONTHLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}monthly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\monthly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        }
    }
}

fn write_daily(conn : &mut SqliteConnection, file : &PathBuf, period : Period, date_interval : (Option<NaiveDate>, Option<NaiveDate>)) {
    let subjects = period.fetch_subjects(conn);
    let mut writer = match csv::Writer::from_path(file){
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to write when exporting: {e}");
            process::exit(1);
        }
    };
    match  writer.write_record(get_header(&subjects)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to write when writing header: {e}");
            process::exit(1);
        }
    }
    let mut i  = date_interval.0.unwrap_or(period.initial_date);
    while i.le(&date_interval.1.unwrap_or(period.final_date)) {
        let mut record : Vec<String> = Vec::new();
        record.push(i.format(FORMAT).to_string());
        for j in &subjects {
            record.push(Entry::get_time_by_day_and_subject(i, j.id, conn).to_string());
        }
        match writer.write_record(record) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to export data: {e}");
                process::exit(1);
            }
        }
        i = i.add(TimeDelta::days(1));
    }
}

pub fn csv_export(conn: &mut SqliteConnection, period : Period, date_interval : (Option<NaiveDate>, Option<NaiveDate>), program_path: &str, mode : ExportMode) {
    let mut descr = period.description.clone();
    descr.truncate(10);
    let path = get_file_path(program_path, &mode, descr);
    match mode {
        ExportMode::DAILY => {
            write_daily(conn, &path, period, date_interval);
            println!("Succesfully exported at {}", path.into_os_string().into_string().unwrap());
        }
        ExportMode::WEEKLY => {

        }
        ExportMode::MONTHLY => {

        }
    }

}