use std::process;
use crate::{plan, status, subject, usage};

pub fn interpret(args: &mut Vec<String>) {
    match args.len() {
        0 => {
            usage::display_usage();
            process::exit(0);
        },
        _ => {
            let option = args.get(0).unwrap().clone();
            args.remove(0);
            match option.trim() {
                "status" => status::get_status(),
                "plan" => plan::interpret(args),
                "subject" => subject::interpret(args),
                _ => {}
            }
        }
    }
}