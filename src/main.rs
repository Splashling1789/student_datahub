mod usage;
mod plan;
mod interpreter;
mod models;
mod schema;
mod error_handler;

use std::env;
use colored::Colorize;

macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
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
