mod core;

use std::path::Path;

use clap::Parser;

/// todo-ci: A simple ci tool to check overdue todos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Flag to disable ignored files by default (.gitignore, hidden files, etc.)
    #[clap(short = 'n', long = "no-ignore", parse(from_flag))]
    no_ignore: bool,
    /// Root directory to check `todos` for
    #[clap(value_parser, default_value = "./")]
    root_directory: String,
}

fn main() {
    let args = Args::parse();

    let matches = core::search_all_files_for_todos(&Path::new(&args.root_directory), args.no_ignore);
    println!("{:?}", matches);
}
