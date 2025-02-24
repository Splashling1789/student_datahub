use crate::interpreter::get_specific_arg;
use crate::plan::interpreter::get_plan_arg;
use crate::plan::period::get_actual_period;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{id, name, short_name};
use crate::subject::interpreter::get_subject;
use crate::subject::usage::display_bad_usage;
use diesel::ExpressionMethods;
use diesel::{update, QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

pub fn modify(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    if args.len() < 2 {
        display_bad_usage();
        process::exit(1);
    }
    let plan_id = get_plan_arg(args, conn);
    let subj = match get_subject(args.get(0).unwrap(), conn, Some(plan_id)) {
        Some(subj) => subj,
        None => {
            eprintln!("Failed to get subject. Does this subject exist?");
            process::exit(1);
        }
    };
    let new_short_name = match get_specific_arg(args, "--short-name") {
        Some(short) => short,
        None => subj.short_name.clone(),
    };
    // Two subjects from the same plan can't have the same short name.
    if super::fetch_all_subjects(conn).iter().any(|s| {
        s.id != subj.id && s.period_id == subj.period_id && s.short_name.eq(&new_short_name)
    }) {
        eprintln!("A subject already exists in the period with the same short name.");
        process::exit(1);
    }
    let new_name = match get_specific_arg(args, "--name") {
        Some(n) => n,
        None => subj.name.clone(),
    };

    match update(subjects.filter(id.eq(subj.id)))
        .set((short_name.eq(new_short_name), name.eq(new_name)))
        .execute(conn)
    {
        Ok(_) => {
            println!("Subjects edited succesfully.");
        }
        Err(e) => {
            eprintln!("Failed to update the subject: {}", e);
            process::exit(1);
        }
    }
}
