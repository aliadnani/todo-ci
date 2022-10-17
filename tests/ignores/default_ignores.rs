use std::path::Path;

use chrono::FixedOffset;
use pretty_assertions::assert_eq;

#[test]
fn find_todos_with_default_ignore_files() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/ignores/fixtures/default").to_path_buf(),
        false,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);
}

#[test]
fn find_todos_without_default_ignore_files() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/ignores/fixtures/default").to_path_buf(),
        true,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    // 3 is the includes the actual .ignore file too
    assert_eq!(search_results.statistics.files_searched, 3);
    assert_eq!(search_results.statistics.valid_todo_count, 2);
}
