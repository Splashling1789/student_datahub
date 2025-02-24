use crate::models::Period;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::initial_date;
use colored::Colorize;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use std::process;

pub fn list(conn: &mut SqliteConnection) {
    let list = match periods.order_by(initial_date).load::<Period>(conn) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to fetch: {e}");
            process::exit(1);
        }
    };
    println!("Study periods:");
    if list.is_empty() {
        println!("No periods created yet.");
    }
    for i in list {
        if i.is_actual() {
            println!("{}", i.to_string().green());
        } else {
            println!("{}", i.to_string());
        }
    }
}
