use std::path::Path;

use chrono::FixedOffset;
use pretty_assertions::assert_eq;

#[test]
/// Test is not too comprehensive as by doing so, we would just be testing ripgrep.
/// We just want to see that it works
fn find_todos_using_a_filename_pattern() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/ignores/fixtures/patterns").to_path_buf(),
        false,
        "*be_scanned*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 2);
    assert_eq!(search_results.statistics.valid_todo_count, 2);
}
