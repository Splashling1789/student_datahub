use crate::debug_println;
use crate::models::Subject;

pub(super) fn weekly_summary(
    total_time_studied: i32,
    times: &Vec<(Subject, i32)>,
    last_week: Option<i32>,
    average: Option<f32>,
) {
    if total_time_studied > 0 {
        println!("\tThis week you have studied {total_time_studied} minutes:");
        for i in times {
            if i.1 != 0 {
                println!("\t * {} minutes were dedicated on {}", i.1, i.0.name);
            }
        }
        if let Some(last_week) = last_week {
            println!();
            if last_week != 0 {
                match total_time_studied as f32 / (last_week as f32) {
                    k @ 0.0..1.0 => {
                        println!(
                            "\t - You've studied {:.1}% less than last week.",
                            (1.0 - k) * 100.0
                        );
                    }
                    1.0 => {
                        println!("\t - You've studied just the same as last week.");
                    }
                    k @ 1.0..=f32::MAX => {
                        println!(
                            "\t - You've studied {:.1}% more than last week.",
                            (k - 1.0) * 100.0
                        );
                    }
                    k => {
                        debug_println!("Unexpected value at division: {k}");
                    }
                }
            } else {
                println!(
                    "\t - Definitely better than last week. (Last week you didn't studied at all)"
                );
            }
        }
        if let Some(average) = average {
            if average != 0.0 {
                match total_time_studied as f32 / average {
                    k @ 0.0..1.0 => {
                        println!("\t - {:.1}% less than average.", (1.0 - k) * 100.0);
                    }
                    1.0 => {}
                    k @ 1.0..=f32::MAX => {
                        println!("\t - {:.1}% more than weekly average.", (k - 1.0) * 100.0);
                    }
                    k => {
                        debug_println!("Unexpected value at division: {k}");
                    }
                }
            }
        }
    } else if total_time_studied == 0 {
        println!("\tYou haven't even study this week either!");
    }
}
