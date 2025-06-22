//! Handles the weekly summary of the status command.
use crate::models::Subject;
use crate::{debug_println, format_hours_and_minutes};

/// Prints the weekly summary of the status command.
/// # Arguments
/// * `total_time_studied` - Total time studied during the week.
/// * `times` - List of (Subject, dedicated time during the week) tuples.
/// * `last_week` - Total time studied the previous week (if any)
/// * `average` - Weekly average so far (if any).
pub(crate) fn weekly_summary(
    total_time_studied: i32,
    times: &Vec<(Subject, i32)>,
    last_week: Option<i32>,
    average: Option<f64>,
) {
    match total_time_studied {
        0 => {
            println!("\tYou haven't even study this week either!");
        }
        total_time_studied => {
            println!(
                "\tThis week you have studied {}:",
                format_hours_and_minutes(total_time_studied)
            );
            for i in times {
                if i.1 != 0 {
                    println!(
                        "\t * {} were dedicated to {}",
                        format_hours_and_minutes(i.1),
                        i.0.name
                    );
                }
            }
            if let Some(last_week) = last_week {
                println!();
                if last_week != 0 {
                    debug_println!("Last week studied: {}", last_week);
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
                debug_println!("avg: {average}");
                if average != 0f64 {
                    match total_time_studied as f64 / average {
                        k @ 0.0..1.0 => {
                            println!("\t - {:.1}% less than average.", (1.0 - k) * 100.0);
                        }
                        1.0 => {}
                        k @ 1.0..=f64::MAX => {
                            println!("\t - {:.1}% more than weekly average.", (k - 1.0) * 100.0);
                        }
                        k => {
                            debug_println!("Unexpected value at division: {k}");
                        }
                    }
                }
            }
        }
    }
}
