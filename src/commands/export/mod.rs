//! # Functionallity of the export command
//! This module handles the export of student data from the datahub into formats such as CSV.

use std::fmt::{Display, Formatter};
use crate::commands::export::usage::display_bad_usage;
use crate::commands::plan::{get_date_arg, get_plan_arg};
use crate::get_data_dir;
use crate::models::Period;
use csv_export::csv_export;
use diesel::SqliteConnection;
use std::process;

mod csv_export;
mod usage;

/// Enum for different export modes.
enum ExportMode {
    /// Export study time day by day.
    Daily,
    /// Export study time week by week.
    Weekly,
    /// Export study time month by month.
    Monthly,
}

impl Display for ExportMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ExportMode::Daily => f.write_str("daily"),
                ExportMode::Weekly => f.write_str("weekly"),
                ExportMode::Monthly => f.write_str("monthly"),
            }
    }
}

/// Interprets export subcommands.
/// # Arguments
/// * `args`: Remaining program arguments.
/// * `conn` : Database connection.
pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    let plan_id = get_plan_arg(args, conn);
    if args.is_empty() || args.first().unwrap().starts_with("--") {
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
    match args.first().unwrap().trim() {
        "daily" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &get_data_dir(),
            ExportMode::Daily,
        ),
        "weekly" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &get_data_dir(),
            ExportMode::Weekly,
        ),
        "monthly" => csv_export(
            conn,
            &period,
            (&start_date, &end_date),
            &get_data_dir(),
            ExportMode::Monthly,
        ),
        "all" => {
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &get_data_dir(),
                ExportMode::Daily,
            );
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &get_data_dir(),
                ExportMode::Weekly,
            );
            csv_export(
                conn,
                &period,
                (&start_date, &end_date),
                &get_data_dir(),
                ExportMode::Monthly,
            );
        }
        _ => {
            display_bad_usage();
            process::exit(1);
        }
    }
}
