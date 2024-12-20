mod cli;
mod task;

use std::path::PathBuf;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use task::Task;
use anyhow::anyhow;


fn find_default_journal_file() -> Option<PathBuf>{
    home::home_dir().map(|mut path |{
        path.push(".rusty-journal.json");
        path
    })
}


fn main() -> anyhow::Result<()> {
    // Get command Line Args
    let CommandLineArgs {
        action,
        journal_file,

    } = CommandLineArgs::from_args();

    // Unpack the journal file
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file !!"))?;

    // Perform the action.
    match action {
        Add { task, priority } => task::add_task(journal_file, Task::new(task, priority)),
        List => task::list_tasks(journal_file),
        Clear => task::clear_tasks(journal_file),
        Done { position} => task::complete_task(journal_file, position),
    }?;

    Ok(())
}
