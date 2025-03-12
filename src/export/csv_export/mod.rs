//! # CSV Export Module
//! CSV exports are generated with the header `["date", subject1.short_name, subject2.short_name, ...]`.
//! The corresponding values below are dates and the respective study time for each date.
//! These 'dates' may also represent intervals, depending on the selected export mode.
mod write_daily;
mod write_monthly;
mod write_weekly;

use super::ExportMode;
use crate::models::{Period, Subject};
use csv::Writer;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::SqliteConnection;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use write_daily::write_daily;
use write_monthly::write_monthly;
use write_weekly::write_weekly;

/// Datetime format for the exported filename.
const DATETIME_FILENAME_EXPORT_FORMAT: &str = "%Y%m%d_%H%M%S";
/// Date format when exporting monthly data.
const MONTHLY_FORMAT: &str = "%m-%Y";

/// Gets a csv header based on the short name of the provided list of subjects.
/// # Arguments
/// * `subjects` - List of subjects.
fn get_header(subjects: &Vec<Subject>) -> Vec<String> {
    let mut header = vec![String::from("date")];
    for i in subjects {
        let column = i.short_name.clone();
        header.push(column);
    }
    header
}

/// Gets the `[csv::Writer]` for the `[PathBuf]` provided, exiting if the writter couldn't be created.
/// # Arguments
/// * `path` - File path.
fn get_csv_writer(path: &PathBuf) -> Writer<File> {
    match csv::Writer::from_path(path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to write when exporting: {e}");
            process::exit(1);
        }
    }
}

/// Gets the csv file path based on a folder path, export mode and description. It will be formatted as `"{path}{ExportMode}_{description}\_{now_datetime}"`
fn get_file_path(path: &str, mode: &ExportMode, descr: String) -> PathBuf {
    return PathBuf::from(format!(
        "{path}{}_{}_{}.csv",
        mode.to_string(),
        descr,
        Local::now()
            .naive_local()
            .format(DATETIME_FILENAME_EXPORT_FORMAT)
            .to_string()
    ));
    #[cfg(target_os = "windows")]
    return PathBuf::from(format!(
        "{path}{}_{}_{}.csv",
        mode.to_string(),
        descr,
        Local::now()
            .naive_local()
            .format(DATETIME_FILENAME_EXPORT_FORMAT)
            .to_string()
    ));
}

/// It exports all the data from a period in a specific interval and export mode to a file in the given path.
/// # Arguments
/// * `conn` - Database connection
/// * `period` - Period from which export the data.
/// * `date_interval` - First day and last day of data.
/// * `program_path` - Folder where will be created the data file.
/// * `mode` - Export mode.
pub fn csv_export(
    conn: &mut SqliteConnection,
    period: &Period,
    date_interval: (&NaiveDate, &NaiveDate),
    program_path: &str,
    mode: ExportMode,
) {
    let mut descr = period.description.clone();
    descr.truncate(10);
    let path = get_file_path(program_path, &mode, descr);
    match mode {
        ExportMode::DAILY => {
            write_daily(conn, &path, period, date_interval);
        }
        ExportMode::WEEKLY => {
            write_weekly(conn, &path, period, date_interval);
        }
        ExportMode::MONTHLY => {
            write_monthly(conn, &path, period, date_interval);
        }
    }
    println!(
        "Succesfully exported at {}",
        path.into_os_string().into_string().unwrap()
    );
}
