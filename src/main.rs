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
    let search = core::Search::new(args.root_directory, args.no_ignore);
    let results = &search.run();

    // Print results of search
    let printer = display::Printer::new(args.display_mode);
    printer.print(results);

    if results.overdue_todos.len() > 0 && !args.no_error {
        std::process::exit(1)
    }
}
