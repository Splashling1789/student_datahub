use diesel::ExpressionMethods;
use diesel::{RunQueryDsl, SqliteConnection};
use std::process;
use diesel::dsl::insert_into;
use crate::plan::interpreter::get_plan_arg;
use crate::subject::usage::display_bad_usage;
use crate::plan::period::{get_actual_period};
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::{name, short_name, period_id};

pub fn add(args : &mut Vec<String>, conn : &mut SqliteConnection) {
    if args.len() < 2 {
        display_bad_usage();
    }
    let new_plan_id = match args.contains(&String::from("--plan")) {
        true => {
            if args.len() < 3 {
                display_bad_usage();
                process::exit(1);
            }
            get_plan_arg(args)
        }
        false => {
            match get_actual_period(conn) {
                Some(p) => p.id,
                None => {
                    eprintln!("No current plan, nor specified.");
                    process::exit(1);
                }
            }
        }
    };
    let (new_short_name, new_name) = match args.get(0).unwrap().trim() {
        "--plan" => {
            (args.get(2).unwrap().clone(), args.get(3).unwrap().clone())
        }
        k => {
            match args.get(1).unwrap().trim() {
                "--plan" => (String::from(k), args.get(3).unwrap().clone()),
                j => (String::from(k), String::from(j)),
            }
        }
    };
    if new_short_name.parse::<i32>().is_ok() {
        eprintln!("Short name can't be a number");
        process::exit(1);
    }

        // Two subjects from the same plan can't have the same short name.
        if super::fetch_all_subjects(conn).iter().any(|s| s.period_id == new_plan_id && s.short_name.eq(&new_short_name)) {
            eprintln!("A subject already exists in the period with the same short name.");
            process::exit(1);
        }

    match insert_into(subjects)
        .values((
        short_name.eq(new_short_name),
        name.eq(new_name),
        period_id.eq(new_plan_id)
    )).execute(conn) {
        Ok(_) => {
            println!("Subject added succesfully");
        }
        Err(e) => {
            eprintln!("Could not insert subject into database: {}", e);
            process::exit(1);
        }
    }
}