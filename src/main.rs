mod cli;
mod core;
mod display;

use clap::Parser;

fn main() {
    // Get CLI args
    let args = cli::Args::parse();

    // Validate and transform CLI args
    // ...

    // Run todo search
    let search_results = core::search(args.root_directory, args.no_ignore);

    // Print results of search
    let printer = display::Printer::new(args.display_mode);
    printer.print(&search_results);

    if !search_results.overdue_todos.is_empty() && !args.no_error {
        std::process::exit(1)
    }
}
