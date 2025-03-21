//! Bad usage display command.

/// Displays the bad usage message from subject command
pub fn display_bad_usage() {
    println!(
        "Bad usage: {} subject ...:\n
        - add [--plan (plan id)] (short name) (name): Adds a new subject to the current/specified plan.
        - modify (id/short name) [--name (new name)] [--short-name (new short name)]: Modifies a subject.
        - remove (id/short name) [--confirm] : Removes a subject.
        - list [--plan (plan id)]: Lists all the subjects from the current/specified period.
        - mark (id/short name) (mark)
        - unmark (id/short name)\n\
    ", crate::env::args().collect::<Vec<String>>().first().unwrap());
}
