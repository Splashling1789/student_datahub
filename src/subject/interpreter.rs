use diesel::QueryDsl;
use std::process;
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};
use diesel::row::NamedRow;
use crate::debug_println;
use crate::interpreter::get_specific_arg;
use crate::models::Subject;
use crate::plan::interpreter::get_plan_arg;
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{id, short_name};
use crate::subject::{add, fetch_all_subjects};
use crate::subject::usage::display_bad_usage;

pub fn get_subject(subject_arg : &String, conn : &mut SqliteConnection, plan_id: Option<i32>) -> Option<Subject> {
    match subject_arg.parse::<i32>() {
        Ok(subject_id) => {
            match subjects.filter(id.eq(subject_id)).load::<Subject>(conn) {
                Ok(s) => {
                    s.first().cloned()
                },
                Err(e) => {
                    debug_println!("Failed to load subject with id {}: {:?}", subject_id, e);
                    None
                }
            }
        }
        Err(_) => {
            match subjects.filter(short_name.eq(subject_arg)).load::<Subject>(conn) {
                Ok(s) => {
                    if(s.len() > 1 && plan_id.is_some()) {
                        debug_println!("There is more than one subject with same short name.");
                        fetch_all_subjects(conn).iter().filter(|s| s.period_id == plan_id.unwrap()).collect::<Vec<Subject>>().first().cloned()
                    }
                    else {
                        s.first().cloned()
                    }
                }
                Err(e) => {
                    debug_println!("Failed to find subject with short name '{}': {:?}", subject_arg, e);
                    None
                }
            }
        }
    }
}

pub fn interpret(args : &mut Vec<String>, conn : &mut SqliteConnection) {
    if args.len() == 0 {
        display_bad_usage()
    }
    else {
        let option = args.get(0).cloned().unwrap();
        args.remove(0);
        match option.trim() {
            "add" => {
                add::add(args, conn);
            }
            "modify" => {

            }
            "remove" => {

            }
            "list" => {

            }
            "mark" => {

            }
            "unmark" => {

            }
            k => {
                debug_println!("No valid argument. Provided: {k}");
                display_bad_usage();
                process::exit(1);
            }
        }
    }
}