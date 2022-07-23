use std::path::Path;

use chrono::FixedOffset;

#[test]
fn default_ignores() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_default").to_path_buf(),
        false,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);
}

#[test]
fn default_ignores_disabled() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_default").to_path_buf(),
        true,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    // 3 is the includes the actual .ignore file too
    assert_eq!(search_results.statistics.files_searched, 3);
    assert_eq!(search_results.statistics.valid_todo_count, 2);
}

#[test]
fn tdignore_always() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_tdignore").to_path_buf(),
        false,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);

    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_tdignore").to_path_buf(),
        true,
        "*".to_string(),
        &FixedOffset::west(0),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 1);
    assert_eq!(search_results.statistics.valid_todo_count, 1);
}
