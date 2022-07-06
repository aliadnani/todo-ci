use std::{fs, path::Path};

use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher},
};
use ignore::{Walk, WalkBuilder};
use time::{format_description::well_known::Iso8601, Date};
use walkdir::WalkDir;

#[derive(Debug)]
/// Returns
pub enum TodoSearchResult {
    Todo {
        file: Box<Path>,
        line_number: i32,
        date: Date,
        description: String,
    },
    ParseError {
        file: Box<Path>,
        line_number: i32,
        error: String,
    },
}

pub fn search_all_files_for_todos(
    root_directory: &Path,
    no_ignore: bool,
) -> Vec<TodoSearchResult> {
    let mut matches: Vec<TodoSearchResult> = vec![];

    let mut builder = WalkBuilder::new(root_directory);

    let walk = builder.ignore(!no_ignore).build();

    for file in walk.into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            matches.append(&mut search_file_for_todos(file.path()))
        }
    }

    matches
}

fn search_file_for_todos(file_path: &Path) -> Vec<TodoSearchResult> {
    // Matches into groups: @todo(<date>):<description>
    // - date: has to be a 10 character string
    // - description: everything after the '):'
    const PATTERN: &str = r"@todo\((?P<date>.{10})\):(?P<description>.*)";

    let matcher = RegexMatcher::new_line_matcher(PATTERN).unwrap();

    let mut searcher = Searcher::new();
    let mut matches: Vec<TodoSearchResult> = vec![];

    let result = searcher
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
                let mut captures = matcher.new_captures().unwrap();
                matcher.captures(line.as_bytes(), &mut captures).unwrap();

                let date_string = &line[captures.get(1).unwrap()];
                let description_string = &line[captures.get(2).unwrap()];

                // Validate date
                let date = match Date::parse(date_string, &Iso8601::DEFAULT) {
                    Ok(date) => date,
                    Err(_) => {
                        matches.push(TodoSearchResult::ParseError {
                            file: file_path.into(),
                            line_number: lnum as i32,
                            error: format!("{} is not a valid date.", date_string),
                        });
                        return Ok(true);
                    }
                };

                matches.push(TodoSearchResult::Todo {
                    file: file_path.into(),
                    line_number: lnum as i32,
                    date: date,
                    description: description_string.to_string(),
                });
                Ok(true)
            }),
        )
        .unwrap();

    matches
}
