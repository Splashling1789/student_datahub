use crate::plan::interpreter::get_plan_arg;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::id;
use crate::subject::interpreter::get_subject;
use crate::subject::usage::display_bad_usage;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{delete, RunQueryDsl, SqliteConnection};
use std::process;

pub fn remove(args : &mut Vec<String>, conn : &mut SqliteConnection) {
    if args.len() < 1 {
        display_bad_usage();
        process::exit(1);
    }
    let plan_id = get_plan_arg(args, conn);
    match get_subject(args.get(0).unwrap(), conn, Some(plan_id)) {
        Some(subj) => {
            match delete(subjects.filter(id.eq(subj.id))).execute(conn) {
                Ok(_) => {
                    //TODO: Request confirmation through stdin
                    println!("Subject removed succesfully");
                }
                Err(e) => {
                    eprintln!("Error deleting subject: {e}");
                    process::exit(1);
                }
            }
        }
        None => {
            eprintln!("There is no subject with such id or name.");
            process::exit(1);
        }
    }

}