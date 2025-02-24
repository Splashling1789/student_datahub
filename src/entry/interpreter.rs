use std::process;
use diesel::SqliteConnection;
use crate::debug_println;
use crate::entry::usage::display_bad_usage;

pub fn interpret(args: &mut Vec<String>, conn: &mut SqliteConnection) {
    if args.len() < 3 {
        display_bad_usage();
    }
    else {
        let option = args.remove(0);
        match option.trim() {
            "add" => {
                
            },
            "substract" => {
                
            },
            k => {
                display_bad_usage();
                debug_println!("Unknown option: {}", k);
                process::exit(1);
            }
        }
    }
}