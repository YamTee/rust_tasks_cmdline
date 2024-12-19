mod cli;
mod task;
use std::path::PathBuf;

use crate::task::Task;
use structopt::StructOpt;

fn main() {
    let task = Task::new("task");

    Task.add_task(PathBuf::new(), task);

    println!("{:#?}", cli::CommandLineArgs::from_args());
}
