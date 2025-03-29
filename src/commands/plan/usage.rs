use crate::FORMAT;

pub fn display_bad_usage() {
    println!(
        "Bad usage: {} plan ...:\n
        - start [start] (end) (description) : Starts a new study plan. It starts today if no start date is provided.
        - list : Lists all the study periods.
        - modify [--plan (plan id)] [--start (new start date)] [--end (new end date)] [--description (new description)] : Modifies the current plan (or one determined by an id).
        - remove [plan id] [--confirm] : Removes the actual study plan (or one determined by id). Use the --confirm option to do so without any warning.
        The date format is: {FORMAT}\n\
    ", crate::env::args().collect::<Vec<String>>().first().unwrap());
}
