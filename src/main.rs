mod cli;
mod tasks;
use structopt::StructOpt;

use tasks::Task;

use cli::{Action::*, CommandLineArgs};

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(&task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
