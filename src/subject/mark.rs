use crate::plan::interpreter::get_plan_arg;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{final_score, id};
use crate::subject::interpreter::get_subject;
use crate::subject::usage::display_bad_usage;
use diesel::ExpressionMethods;
use diesel::{update, QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

pub fn update_mark(args : &mut Vec<String>, conn : &mut SqliteConnection, unmark : bool) {
    if (args.len() < 2 && !unmark) || (args.len() < 1 && unmark) {
        display_bad_usage();
        process::exit(1);
    }
    let plan_id = get_plan_arg(args, conn);
    match get_subject(args.get(0).unwrap(), conn, Some(plan_id)) {
        Some(s) => {
            if !unmark {
                let mark = match args.get(1).unwrap().parse::<f32>() {
                    Ok(m) => m,
                    Err(_) => {
                        eprintln!("Mark must be a decimal or integer number (e.g.: 5.2)");
                        process::exit(1);
                    }
                };
                match update(subjects.filter(id.eq(s.id))).set(final_score.eq(mark)).execute(conn) {
                    Ok(_) => {
                        println!("Successfully marked {} with score {}", s.short_name, mark);
                    },
                    Err(e) => {
                        eprintln!("Failed marking: {e}");
                        process::exit(1);
                    }
                }
            }
            else {
                match update(subjects.filter(id.eq(s.id))).set(final_score.eq::<Option<f32>>(None)).execute(conn) {
                    Ok(_) => {
                        println!("Successfully unmarked {}", s.short_name);
                    },
                    Err(e) => {
                        eprintln!("Failed marking: {e}");
                        process::exit(1);
                    }
                }
            }

        }
        None => {
            println!("No such subject");
            process::exit(1);
        }
    }
}