mod write_daily;
mod write_monthly;
mod write_weekly;

use std::fs::File;
use std::path::PathBuf;
use std::process;
use csv::Writer;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate, Weekday};
use diesel::SqliteConnection;
use write_daily::write_daily;
use write_monthly::write_monthly;
use write_weekly::write_weekly;
use super::ExportMode;
use crate::models::{Period, Subject};

const DATETIME_FILENAME_EXPORT_FORMAT: &str = "%Y%m%d_%H%M%S";
const WEEKDAY_START : Weekday = Weekday::Mon;
const MONTHLY_FORMAT : &str = "%m-%Y";


fn get_header(subjects : &Vec<Subject>) -> Vec<String> {
    let mut header = vec![String::from("date")];
    for i in subjects {
        let column = i.short_name.clone();
        header.push(column);
    }
    header
}

fn get_csv_writer(path : &PathBuf) -> Writer<File> {
    match csv::Writer::from_path(path){
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to write when exporting: {e}");
            process::exit(1);
        }
    }
}

fn get_file_path(path: &str, mode : &ExportMode, descr : String) -> PathBuf {
    match mode {
        ExportMode::DAILY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}daily_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::WEEKLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}weekly_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}weekly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
        },
        ExportMode::MONTHLY => {
            #[cfg(not(target_os = "windows"))]
            return PathBuf::from(format!("{path}monthly_{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
            #[cfg(target_os = "windows")]
            return PathBuf::from(format!("{path}monthly{}_{}.csv", descr, Local::now().naive_local().format(DATETIME_FILENAME_EXPORT_FORMAT).to_string()));
        }
    }
}

pub fn csv_export(conn: &mut SqliteConnection, period : &Period, date_interval : (&NaiveDate, &NaiveDate), program_path: &str, mode : ExportMode) {
    let mut descr = period.description.clone();
    descr.truncate(10);
    let path = get_file_path(program_path, &mode, descr);
    match mode {
        ExportMode::DAILY => {
            write_daily(conn, &path, period, date_interval);
            println!("Succesfully exported at {}", path.into_os_string().into_string().unwrap());
        }
        ExportMode::WEEKLY => {
            write_weekly(conn, &path, period, date_interval);
            println!("Succesfully exported at {}", path.into_os_string().into_string().unwrap());
        }
        ExportMode::MONTHLY => {
            write_monthly(conn, &path, period, date_interval);
            println!("Succesfully exported at {}", path.into_os_string().into_string().unwrap());
        }
    }
}