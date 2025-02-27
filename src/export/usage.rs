use crate::FORMAT;

pub fn display_bad_usage() {
    println!(
        "Bad usage: {} export (mode) [--plan (plan id)] [--start (start date)] [--end (end date)]:\n\
        Avaliable modes:\n\n
            \t- daily: One line is one day\n\
            \t- weekly: One line is one week\n\
            \t- monthly: One line is one month\n\
            \t- all: Three files for every previous mode\n\
        The date format is: {FORMAT}\n\
    ",
        crate::env::args().collect::<Vec<String>>().first().unwrap()
    );
}
