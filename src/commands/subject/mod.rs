//! Handles commands related to subjects.

mod add;
mod list;
mod mark;
mod modify;
mod remove;
mod usage;

use crate::commands::plan::get_plan_arg;
use crate::commands::subject::usage::display_bad_usage;
use crate::debug_println;
use crate::interpreter::{detect_unknown_arg, get_specific_arg, request_confirmation};
use crate::models::Subject;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{id, short_name};
use diesel::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};
use std::process;

/// It interprets a Subject argument. If the argument is a number, it will fetch it by id. If it is not a number, it will
/// fetch it by short name, using the provided plan_id.
/// # Arguments
/// * `subject_arg` - Subject argument
/// * `conn` - Database connection
/// * `plan_id` - Plan id in order to fetch by short name
pub fn get_subject(
    subject_arg: &String,
    conn: &mut SqliteConnection,
    plan_id: Option<i32>,
) -> Option<Subject> {
    match subject_arg.parse::<i32>() {
        Ok(subject_id) => match subjects.filter(id.eq(subject_id)).load::<Subject>(conn) {
            Ok(s) => s.first().cloned(),
            Err(e) => {
                debug_println!("Failed to load subject with id {}: {:?}", subject_id, e);
                None
            }
        },
        Err(_) => {
            match subjects
                .filter(short_name.eq(subject_arg))
                .load::<Subject>(conn)
            {
                Ok(s) => {
                    if s.len() > 1 && plan_id.is_some() {
                        debug_println!("There is more than one subject with same short name.");
                        s.iter().find(|s| s.period_id == plan_id.unwrap()).cloned()
                    } else {
                        s.first().cloned()
                    }
                }
                Err(e) => {
                    debug_println!(
                        "Failed to find subject with short name '{}': {:?}",
                        subject_arg,
                        e
                    );
                    None
                }
            }
        }
    }
}

/// Interprets subject subcommands.
/// # Arguments
/// * `args` - Remaining program arguments.
/// * `conn` - Database connection.
pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    if args.is_empty() {
        display_bad_usage()
    } else {
        let option = args.first().cloned().unwrap();
        args.remove(0);
        let plan_id = get_plan_arg(args, conn);
        match option.trim() {
            "add" => {
                if args.len() < 2 {
                    display_bad_usage();
                    process::exit(1);
                }
                let (new_short_name, new_name) =
                    (args.first().unwrap().clone(), args.get(1).unwrap().clone());
                if new_short_name.parse::<i32>().is_ok() {
                    eprintln!("Short name can't be a number");
                    process::exit(1);
                }
                add::add(conn, plan_id, new_short_name, new_name);
            }
            "modify" => {
                if args.is_empty() {
                    display_bad_usage();
                    process::exit(1);
                }
                if let Some(o) = detect_unknown_arg(args, &vec!["--name", "--short-name"], "--") {
                    eprintln!("Unknown argument: {o}");
                    display_bad_usage();
                    process::exit(1);
                }
                let subj = match get_subject(args.first().unwrap(), conn, Some(plan_id)) {
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
                let new_name = match get_specific_arg(args, "--name") {
                    Some(n) => n,
                    None => subj.name.clone(),
                };
                modify::modify(conn, subj, new_short_name, new_name);
            }
            "remove" => {
                if args.is_empty() {
                    display_bad_usage();
                    process::exit(1);
                }
                match get_subject(args.first().unwrap(), conn, Some(plan_id)) {
                    Some(subj) => {
                        if !args.contains(&"--confirm".to_string()) {
                            println!("{}", subj);
                            request_confirmation(
                                "Are you sure you want to delete this subject? [y/n]",
                            );
                        }
                        remove::remove(conn, subj.id);
                    }
                    None => {
                        eprintln!("There is no subject with such id or name.");
                        process::exit(1);
                    }
                }
            }
            "list" => {
                list::list(conn, plan_id);
            }
            option @ "mark" | option @ "unmark" => {
                if args.len() < 2 {
                    display_bad_usage();
                    process::exit(1);
                }
                let subject = match get_subject(args.first().unwrap(), conn, Some(plan_id)) {
                    Some(subj) => subj,
                    None => {
                        eprintln!("Failed to get subject. Does this subject exist?");
                        process::exit(1);
                    }
                };
                match option {
                    "mark" => {
                        let mark = match args.get(1).unwrap().parse::<f32>() {
                            Ok(m) => m,
                            Err(_) => {
                                eprintln!("Mark must be a decimal or integer number (e.g.: 5.2)");
                                process::exit(1);
                            }
                        };
                        mark::update_mark(conn, subject, Some(mark));
                    }
                    "unmark" => {
                        mark::update_mark(conn, subject, None);
                    }
                    _ => {}
                }
            }
            k => {
                debug_println!("No valid argument. Provided: {k}");
                display_bad_usage();
                process::exit(1);
            }
        }
    }
}
