use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
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
pub struct TodoSearchResult {
    pub valid_todos: Vec<Todo>,
    pub overdue_todos: Vec<Todo>,
    pub malformed_todos: Vec<MalformedTodo>,
}

#[derive(Debug)]
pub struct TotalTodoSearchResult {
    pub files_searched: i32,
    pub valid_todos: Vec<Todo>,
    pub overdue_todos: Vec<Todo>,
    pub malformed_todos: Vec<MalformedTodo>,
}


pub fn search(root_directory: PathBuf, no_ignore: bool) -> TotalTodoSearchResult {
    let mut result = TodoSearchResult {
        valid_todos: vec![],
        overdue_todos: vec![],
        malformed_todos: vec![],
    };

    let mut file_count = 0;
    walk_files_and(|file| {
        // TODO: Handle failures
        if file.metadata().unwrap().is_file() {
            let todos = &mut search_todos(file.path());
            file_count += 1;
            result.overdue_todos.append(&mut todos.overdue_todos);
            result.valid_todos.append(&mut todos.valid_todos);
            result.malformed_todos.append(&mut todos.malformed_todos);
        }
    }, root_directory, no_ignore);

    TotalTodoSearchResult {
        files_searched: file_count,
        valid_todos: result.valid_todos,
        overdue_todos: result.overdue_todos,
        malformed_todos: result.malformed_todos,
    }
}

fn walk_files_and<F>(mut f: F, root_directory: PathBuf, no_ignore: bool)
where
    F: FnMut(DirEntry),
{
    let mut builder = WalkBuilder::new(&root_directory);
    let walk = builder.standard_filters(!no_ignore).build();

    for file in walk.into_iter().filter_map(|file| file.ok()) {
        f(file)
    }
}

fn search_todos(file_path: &Path) -> TodoSearchResult {
    // Matches into groups: @todo(<date>):<description>
    // - date: has to be a 10 character string, no validation - I don't want to do it in regex
    // - description: everything after the '):'
    const PATTERN: &str = r"@todo\((?P<date>.{10})\):(?P<description>.*)";

    // TODO: Handle failures
    let matcher = RegexMatcher::new_line_matcher(PATTERN).unwrap();

    let mut searcher = Searcher::new();
    let mut valid_todos: Vec<Todo> = vec![];
    let mut overdue_todos: Vec<Todo> = vec![];
    let mut malformed_todos: Vec<MalformedTodo> = vec![];

    searcher
        .search_path(
            &matcher,
            file_path,
            UTF8(|lnum, line| {
                // Regex group match validation
                if matcher.capture_count() != 3
                    || matcher.capture_index("date") != Some(1)
                    || matcher.capture_index("description") != Some(2)
                {
                    return Ok(true);
                }

                // Parse date & description of 'todo'
                // TODO: Handle failures
                let mut captures = matcher.new_captures().unwrap();
                matcher.captures(line.as_bytes(), &mut captures).unwrap();

                // TODO: Handle failures
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
        )
        // TODO: Handle failures
        .unwrap();

    TodoSearchResult {
        valid_todos,
        overdue_todos,
        malformed_todos,
    }
}
