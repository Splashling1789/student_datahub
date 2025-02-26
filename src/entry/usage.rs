use crate::FORMAT;
pub fn display_bad_usage() {
    println!(
        "Bad usage: {} add/subtract/set [when] (subject id or short name) (amount):\n
        The date format is: {FORMAT}\n\
    ",
        crate::env::args().collect::<Vec<String>>().first().unwrap()
    );
}
