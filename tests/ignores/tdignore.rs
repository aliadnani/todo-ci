use std::path::Path;

use chrono::FixedOffset;
use pretty_assertions::assert_eq;

#[test]
fn test_that_tdignore_cannot_be_disabled() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/ignores/fixtures/tdignore").to_path_buf(),
        false,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);

    let search_results = todo_ci::core::search(
        Path::new("./tests/ignores/fixtures/tdignore").to_path_buf(),
        true,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);
}
