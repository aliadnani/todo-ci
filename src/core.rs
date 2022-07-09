use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
use eyre::Result;
use globset::Glob;
use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use ignore::{DirEntry, WalkBuilder};

#[derive(Debug)]
pub struct Todo {
    pub file: PathBuf,
    pub line_number: i32,
    pub date: NaiveDate,
    pub description: String,
}

#[derive(Debug)]
pub struct MalformedTodo {
    pub file: PathBuf,
    pub line_number: i32,
    pub error: String,
}

#[derive(Debug)]
pub struct TotalSearchResult {
    pub files_searched: i32,
    pub valid_todos: Vec<Todo>,
    pub overdue_todos: Vec<Todo>,
    pub malformed_todos: Vec<MalformedTodo>,
}

#[derive(Debug)]
struct FileSearchResult {
    valid_todos: Vec<Todo>,
    overdue_todos: Vec<Todo>,
    malformed_todos: Vec<MalformedTodo>,
}

pub fn search(
    root_directory: PathBuf,
    no_ignore: bool,
    ignore_pattern: String,
) -> Result<TotalSearchResult> {
    let mut result = FileSearchResult {
        valid_todos: vec![],
        overdue_todos: vec![],
        malformed_todos: vec![],
    };

    let ignore_glob = Glob::new(&ignore_pattern)?.compile_matcher();

    let mut file_count = 0;
    walk_files_and(
        |file| {
            // TODO: Handle failures
            if file.metadata()?.is_file() && ignore_glob.is_match(file.path()) {
                let todos = &mut search_todos(file.path())?;
                file_count += 1;
                result.overdue_todos.append(&mut todos.overdue_todos);
                result.valid_todos.append(&mut todos.valid_todos);
                result.malformed_todos.append(&mut todos.malformed_todos);
            };
            Ok(())
        },
        root_directory,
        no_ignore,
    )?;

    Ok(TotalSearchResult {
        files_searched: file_count,
        valid_todos: result.valid_todos,
        overdue_todos: result.overdue_todos,
        malformed_todos: result.malformed_todos,
    })
}

fn walk_files_and<F>(mut f: F, root_directory: PathBuf, no_ignore: bool) -> Result<()>
where
    F: FnMut(DirEntry) -> Result<()>,
{
    let mut builder = WalkBuilder::new(&root_directory);
    let walk = builder
        .standard_filters(!no_ignore)
        .add_custom_ignore_filename(".tdignore")
        .build();

    for file in walk.into_iter().filter_map(|file| file.ok()) {
        f(file)?;
    }
    Ok(())
}

/// Matches TODOs that follows the format: @todo(<date>):<description>
fn search_todos(file_path: &Path) -> Result<FileSearchResult> {
    const PATTERN: &str = r"@todo\((?P<date>.{10})\):(?P<description>.*)";

    // TODO: Handle failures
    let matcher = RegexMatcher::new_line_matcher(PATTERN)?;

    let mut searcher = Searcher::new();
    let mut valid_todos: Vec<Todo> = vec![];
    let mut overdue_todos: Vec<Todo> = vec![];
    let mut malformed_todos: Vec<MalformedTodo> = vec![];

    searcher.search_path(
        &matcher,
        file_path,
        UTF8(|lnum, line| {
            // Regex group match validation
            if matcher.capture_count() != 3
                || matcher.capture_index("date") != Some(1)
                || matcher.capture_index("description") != Some(2)
            {
                // Ok(true) is used to early return from search sink
                return Ok(true);
            }

            // Parse date & description of 'todo'
            let mut captures = matcher.new_captures()?;
            matcher.captures(line.as_bytes(), &mut captures)?;

            // Unwraps here are ok - as we've already verified 3 capture groups
            let date_string = &line[captures.get(1).unwrap()];
            let description_string = &line[captures.get(2).unwrap()].trim();

            // Validate date
            let date = match NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    malformed_todos.push(MalformedTodo {
                        file: file_path.into(),
                        line_number: lnum as i32,
                        error: format!("{} is not a valid date.", date_string),
                    });
                    return Ok(true);
                }
            };

            match Utc::now().date().naive_utc().cmp(&date) {
                std::cmp::Ordering::Greater => overdue_todos.push(Todo {
                    file: file_path.into(),
                    line_number: lnum as i32,
                    date,
                    description: description_string.to_string(),
                }),
                _ => valid_todos.push(Todo {
                    file: file_path.into(),
                    line_number: lnum as i32,
                    date,
                    description: description_string.to_string(),
                }),
            }

            Ok(true)
        }),
    )?;

    Ok(FileSearchResult {
        valid_todos,
        overdue_todos,
        malformed_todos,
    })
}
