use std::path::Path;

use chrono::{FixedOffset, NaiveDate};
use todo_ci::core::TodoState;

use pretty_assertions::assert_eq;

#[test]
fn find_valid_todos() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/simple/fixtures").to_path_buf(),
        false,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 3);

    // Valid TODO on line 4
    assert_eq!(search_results.statistics.valid_todo_count, 1);
    let valid_todo = search_results
        .todos
        .iter()
        .find(|td| matches!(td.state, TodoState::Valid))
        .unwrap();

    assert_eq!(
        valid_todo.file.as_path(),
        Path::new("./tests/simple/fixtures/file_with_todos.rs")
    );
    assert_eq!(valid_todo.date.unwrap(), NaiveDate::from_ymd(2221, 7, 10));
    assert_eq!(valid_todo.description, "I am a valid TODO on line 4");
    assert_eq!(valid_todo.line_number, 4);
}
