//! # Functionallity of the export command
//! This module handles the export of student data from the datahub into formats such as CSV.
use crate::export::usage::display_bad_usage;
use crate::interpreter::get_data_dir;
use crate::models::Period;
use crate::plan::{get_date_arg, get_plan_arg};
use csv_export::csv_export;
use diesel::SqliteConnection;
use std::process;

mod csv_export;
mod usage;

/// Enum for different export modes.
/// * `DAILY`: Export study time day by day.
/// * `WEEKLY`: Export study time week by week.
/// * `MONTHLY`: Export study time month by month.
enum ExportMode {
    DAILY,
    WEEKLY,
    MONTHLY,
}

impl ExportMode {
    /// Gets the export mode name in lowercase.
    fn to_string(&self) -> String {
        match self {
            ExportMode::DAILY => String::from("daily"),
            ExportMode::WEEKLY => String::from("weekly"),
            ExportMode::MONTHLY => String::from("monthly"),
        }
    }
}

/// Interprets export subcommands.
/// # Arguments
/// * `args`: Remaining program arguments.
/// * `conn` : Database connection.
pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    let plan_id = get_plan_arg(args, conn);
    if args.is_empty() || args.get(0).unwrap().starts_with("--") {
        display_bad_usage();
        process::exit(1);
    }
    let period = match Period::from_id(conn, plan_id) {
        Some(period) => period,
        None => {
            eprintln!("There is no period with the provided id");
            process::exit(1);
        }
    };
    let start_date = match args.contains(&"--start".to_string()) {
        true => get_date_arg(args, "--start"),
        false => period.initial_date,
    };
    let end_date = match args.contains(&"--end".to_string()) {
        true => get_date_arg(args, "--end"),
        false => period.final_date,
    };
    let mut header = vec![String::from("date")];
    let subjects = period.fetch_subjects(conn);
    for i in &subjects {
        let column = i.short_name.clone();
        header.push(column);
    }
    match args.get(0).unwrap().trim() {
        "daily" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &*get_data_dir(),
            ExportMode::DAILY,
        ),
        "weekly" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &*get_data_dir(),
            ExportMode::WEEKLY,
        ),
        "monthly" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &*get_data_dir(),
            ExportMode::MONTHLY,
        ),
        "all" => {
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &*get_data_dir(),
                ExportMode::DAILY,
            );
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &*get_data_dir(),
                ExportMode::WEEKLY,
            );
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &*get_data_dir(),
                ExportMode::MONTHLY,
            );
        }
        _ => {
            display_bad_usage();
            process::exit(1);
        }
    }
}
