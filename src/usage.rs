fn get_usage_string() -> String {
    format!(
        "USAGE: {} subcommand [options]\n\n\
        Avaliable subcommands:\n\
        status: Shows the current data of the study period.\n\
        add/substract [when] (which subject) (minutes): Inserts/substracts a time entry.\n\
        plan list : Shows a list of all the study periods\n\
        plan start [start] (end) (description): Starts a new study period.\n\
        plan modify : Modifies current study period.\n\
        plan remove [--confirm] : Deletes current study period.\n\
        subject add (short name) (long name...): Adds a new subject to the study plan.\n\
        subject modify (short name/id): Modifies a subject from the currect study plan.\n\
        subject remove (short name/id) [--confirm]: Removes a subject from the current study plan. --confirm does not require confirmation\n\
        subject list: Shows the list of subjects from the current study plan.\n\
        ", crate::env::args().collect::<Vec<String>>().first().unwrap())
}

pub fn display_usage() {
    println!("{}", get_usage_string());
}
