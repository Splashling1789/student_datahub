use crate::models::{Period, Subject};
use crate::plan::interpreter::get_plan_arg;
use crate::subject::fetch_all_subjects;
use diesel::SqliteConnection;
use std::process;

pub fn list(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    let plan_id = get_plan_arg(args, conn);
    let plan = match Period::fetch_all_plans(conn)
        .iter()
        .filter(|p| p.id == plan_id)
        .nth(0)
    {
        Some(plan) => plan.clone(),
        None => {
            eprintln!("Failed fetching plan. Does this plan exists?");
            process::exit(1);
        }
    };
    println!(
        "Subjects from period {} ({} - {})",
        plan.description,
        plan.initial_date.to_string(),
        plan.final_date.to_string()
    );
    let subjects_from_plan = fetch_all_subjects(conn)
        .iter()
        .filter(|s| s.period_id == plan_id)
        .cloned()
        .collect::<Vec<Subject>>();
    if subjects_from_plan.is_empty() {
        println!("No subjects from this period");
    } else {
        for s in subjects_from_plan {
            println!("{}, TDT: {}", s.to_string(), s.total_dedicated_time(conn));
        }
    }
}
