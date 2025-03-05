use std::collections::LinkedList;
use std::fmt::Display;
use std::ops::Add;
use std::path::PathBuf;
use std::process;
use diesel::internal::derives::multiconnection::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeDelta};
use diesel::SqliteConnection;
use crate::FORMAT;
use crate::models::{Entry, Period, Subject};

const DATETIME_EXPORT_FORMAT : &str = "%Y%m%d_%H%M%S";

enum ExportMode{
    DAILY,
    WEEKLY,
    MONTHLY
}

fn get_header(subjects : &Vec<Subject>) -> Vec<String> {
    let mut header = vec![String::from("date")];
    for i in subjects {
        let column = i.short_name.clone();
        header.push(column);
    }
    header
}

fn get_file_path(path: &str, mode : ExportMode, descr : String) -> PathBuf {
    match mode {
        ExportMode::DAILY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}/daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::WEEKLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}/weekly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\weekly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::MONTHLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}/monthly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}\\monthly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
        }
    }
}

fn write_daily(conn : &mut SqliteConnection, file : PathBuf, period : Period, date_interval : (Option<NaiveDate>, Option<NaiveDate>)) {
    let subjects = period.fetch_subjects(conn);
    let mut writer = match csv::Writer::from_path(file){
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to write when exporting: {e}");
            process::exit(1);
        }
    };
    writer.write_record(get_header(&subjects)).expect("Failed to write header");
    let mut i  = date_interval.0.unwrap_or(period.initial_date);
    while i.le(&date_interval.1.unwrap_or(period.final_date)) {
        let mut record : Vec<String> = Vec::new();
        for j in &subjects {
            record.push(Entry::get_time_by_day_and_subject(i, j.id, conn).to_string());
        }
        writer.write_record(record).expect("Failed to write record");
        i = i.add(TimeDelta::days(1));
    }
}

fn csv_export(conn: &mut SqliteConnection, period : Period, date_interval : (Option<NaiveDate>, Option<NaiveDate>), path: &str , mode : ExportMode) {
    match mode {
        ExportMode::DAILY => {
            #[cfg(not(target_os = "windows"))]
            let file = PathBuf::from(format!("{path}/daily_{}_{}.csv", period.description, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            let file = PathBuf::from(format!("{path}\\daily_{}_{}.csv", period.description, Local::now().naive_local().format(DATETIME_EXPORT_FORMAT).to_string()));
            
            write_daily(conn, file, period, date_interval);
        }
        ExportMode::WEEKLY => {
            
        }
        ExportMode::MONTHLY => {
            
        }
    }
}