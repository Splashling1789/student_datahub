
fn get_usage_string() -> String{
    format!(
        "USAGE: datahub subcommand [options]\n\n\
        Avaliable subcommands:\n\
        status: Shows the current data of the study period.\n\
        add/substract (when) (which subject) (minutes): Inserts/substracts a time entry.\n\
        plan start: Starts a new study period.\n\
        plan modify: Modifies current study period.\n\
        subject add (short name) (long name...): Adds a new subject to the study plan.\n\
        subject modify (short name/id): Modifies a subject from the currect study plan.\n\
        subject remove (short name/id) [--confirm]: Removes a subject from the current study plan. --confirm does not require confirmation\n\
        subject list: Shows the list of subjects from the current study plan.\n\
        ")
}

pub fn display_usage() {
    println!("{}", get_usage_string());
}