use crate::models::Subject;

pub(super) fn daily_summary(total_time_studied: i32, times : &Vec<(Subject, i32)>) {
    if total_time_studied > 0 {
        println!("\tYou have studied a total amount of {total_time_studied} minutes:");
        for i in times {
            if i.1 != 0 {
                println!("\t * {} minutes were dedicated on {}", i.1, i.0.name);
            }
        }
    }
    else {
        println!("\tYou haven't studied today yet!");
    }
}