use std::process;

pub fn connection_error_handler() {
    eprintln!("Error connecting to the database");
    process::exit(1);
}
