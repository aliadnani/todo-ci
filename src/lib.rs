pub mod cli;
pub mod core;
pub mod display;

use clap::Parser;
use eyre::Result;

pub fn run() -> Result<()> {
    // Get CLI args
    let args = cli::Args::parse();

    // Run todo search
    let search_results = core::search(args.root_directory, args.no_ignore, args.ignore_pattern)?;

    // Print results of search
    display::print(args.display_mode, &search_results);

    if !search_results.overdue_todos.is_empty() && !args.no_error {
        std::process::exit(1)
    };

    Ok(())
}
