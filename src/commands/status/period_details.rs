//! Handles the header of the status command (period details and date).
use crate::models::Period;
use crate::FORMAT;
use colored::Colorize;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;

/// Prints the details of the given period with a given date.
/// # Arguments
/// * `period` - Period to print its details.
/// * `date` - Date to print with the period details.
pub fn print_period_details(period: &Period, date: &NaiveDate) {
    if *date == period.initial_date {
        print!("{}", date.format(FORMAT).to_string().green())
    } else {
        print!("{}", period.initial_date.format(FORMAT));
    }

    match (*date - period.initial_date).num_days() {
        1 => print!(" - {}", date.format(FORMAT).to_string().green()),
        _ => print!(" - ... - {}", date.format(FORMAT).to_string().green()),
    }

    match (period.final_date - *date).num_days() {
        1 => print!(" - {}", period.final_date.format(FORMAT)),
        _ => print!(" - ... - {}", period.final_date.format(FORMAT)),
    }
    println!();
}
