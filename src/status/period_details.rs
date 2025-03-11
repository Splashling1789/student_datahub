use colored::Colorize;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use terminal_size::{terminal_size, Width};
use crate::FORMAT;
use crate::models::Period;

pub fn print_period_details(period : &Period, date : &NaiveDate) {
    if *date == period.initial_date {
        print!("{}", date.format(FORMAT).to_string().green())
    }
    else {
        print!("{}", period.initial_date.format(FORMAT).to_string());
    }

    if (*date - period.initial_date).num_days() == 1 {
        print!(" - {}", date.format(FORMAT).to_string().green());
    }
    else if (*date - period.initial_date).num_days() > 1 {
        print!(" - ... - {}", date.format(FORMAT).to_string().green());
    }

    if (period.final_date - *date).num_days() == 1 {
        print!(" - {}", period.final_date.format(FORMAT).to_string());
    }
    else if (period.final_date - *date).num_days() > 1 {
        print!(" - ... - {}", period.final_date.format(FORMAT).to_string());
    }
    println!();
    
        let line_longitude = match terminal_size() {
            Some((Width(w), _)) => w as i32 / 3,
            None => 30,
        };
        for _ in 0..line_longitude {
            print!("-");
        }
        println!();
    
}