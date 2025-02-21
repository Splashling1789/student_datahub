mod plan;
mod interpreter;
mod usage;
mod models;
mod schema;

use std::env;

pub const FORMAT: &str = "%m-%d-%Y";

macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            use colored::Colorize;
            println!("{}", format!("[DEBUG] {}", format!($($arg)*)).yellow());
        }
    };
}
pub(crate) use debug_println;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    debug_println!("args: {:?}", args);
    interpreter::interpret(&mut args);
}
