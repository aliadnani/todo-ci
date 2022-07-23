use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

use chrono::{FixedOffset, NaiveDate, TimeZone, Utc};
use eyre::Result;
use globset::Glob;
use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use ignore::{DirEntry, WalkBuilder};

#[derive(Debug)]
pub enum TodoState {
    Valid,
    Overdue,
    Malformed,
}

#[derive(Debug)]
pub struct Todo {
    pub file: PathBuf,
    pub line_number: i32,
    pub date: Option<NaiveDate>,
    pub description: String,
    pub state: TodoState,
}

#[derive(Debug)]
pub struct SearchResult {
    pub todos: Vec<Todo>,
    pub statistics: TodoStatistics,
}

#[derive(Debug)]
pub struct TodoStatistics {
    pub files_searched: i32,
    pub valid_todo_count: i32,
    pub overdue_todo_count: i32,
    pub malformed_todo_count: i32,
}

pub fn search(
    root_directory: PathBuf,
    no_ignore: bool,
    ignore_pattern: String,
    fixed_offset: &FixedOffset,
) -> Result<SearchResult> {
    let mut todos: Vec<Todo> = vec![];
    let mut statistics = TodoStatistics {
        files_searched: 0,
        valid_todo_count: 0,
        overdue_todo_count: 0,
        malformed_todo_count: 0,
    };

    let ignore_glob = Glob::new(&ignore_pattern)?.compile_matcher();

    walk_files_and(
        |file| {
            if file.metadata()?.is_file()
                && ignore_glob.is_match(file.path())
                // Prevents .tdignore from being searched as well
                // Not sure how this can be done more elegantly
                && file.path().file_name().unwrap() != ".tdignore"
            {
                let file_search_result = &mut search_todos(file.path(), fixed_offset)?;

                // Aggregate statistics
                statistics.files_searched += 1;
                statistics.valid_todo_count += file_search_result.statistics.valid_todo_count;
                statistics.overdue_todo_count += file_search_result.statistics.overdue_todo_count;
                statistics.malformed_todo_count +=
                    file_search_result.statistics.malformed_todo_count;

                // Aggregate TODOs
                todos.append(&mut file_search_result.todos)
            };
            Ok(())
        },
        root_directory,
        no_ignore,
    )?;

    Ok(SearchResult { todos, statistics })
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

/// Searches for TODOs in a file as well as their statistics
///
/// Matches TODOs that follows the format: @todo(<date>):<description>
fn search_todos(file_path: &Path, fixed_offset: &FixedOffset) -> Result<SearchResult> {
    const PATTERN: &str = r"@todo\((?P<date>.{10})\):(?P<description>.*)";

    let matcher = RegexMatcher::new_line_matcher(PATTERN)?;

    let mut searcher = Searcher::new();
    let mut todos: Vec<Todo> = vec![];
    let mut valid_todo_count = 0;
    let mut overdue_todo_count = 0;
    let mut malformed_todo_count = 0;

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
                Ok(date) => fixed_offset.from_local_date(&date).unwrap(),
                Err(_) => {
                    todos.push(Todo {
                        file: file_path.into(),
                        line_number: lnum as i32,
                        date: None,
                        description: format!("{} is not a valid date.", date_string),
                        state: TodoState::Malformed,
                    });
                    malformed_todo_count += 1;
                    return Ok(true);
                }
            };

            // match Utc::now().date().naive_utc().cmp(&date_time) {
            match Utc::now().with_timezone(fixed_offset).date().cmp(&date) {
                Ordering::Greater => {
                    todos.push(Todo {
                        file: file_path.into(),
                        line_number: lnum as i32,
                        date: Some(date.naive_local()),
                        description: description_string.to_string(),
                        state: TodoState::Overdue,
                    });
                    overdue_todo_count += 1;
                }
                _ => {
                    todos.push(Todo {
                        file: file_path.into(),
                        line_number: lnum as i32,
                        date: Some(date.naive_local()),
                        description: description_string.to_string(),
                        state: TodoState::Valid,
                    });
                    valid_todo_count += 1;
                }
            }

            Ok(true)
        }),
    )?;

    Ok(SearchResult {
        todos,
        statistics: TodoStatistics {
            files_searched: 1,
            valid_todo_count,
            overdue_todo_count,
            malformed_todo_count,
        },
    })
}
