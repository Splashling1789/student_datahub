use crate::format_hours_and_minutes;
use crate::models::Subject;

pub(super) fn daily_summary(total_time_studied: i32, times: &Vec<(Subject, i32)>) {
    if total_time_studied > 0 {
        println!("\tYou have studied a total amount of {}:", format_hours_and_minutes(total_time_studied));
        for i in times {
            if i.1 != 0 {
                println!("\t * {} were dedicated on {}", format_hours_and_minutes(i.1), i.0.name);
            }
        }
    } else {
        println!("\tYou haven't studied today yet!");
    }
}
