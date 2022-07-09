use std::path::Path;

#[test]
fn default_ignores() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_default").to_path_buf(),
        false,
        "*".to_string(),
    )
    .unwrap();

    assert_eq!(search_results.files_searched, 1);
    assert_eq!(search_results.valid_todos.len(), 1);
}


#[test]
fn default_ignores_disabled() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_default").to_path_buf(),
        true,
        "*".to_string(),
    )
    .unwrap();

    // 3 is the includes the actual .ignore file too
    assert_eq!(search_results.files_searched, 3);
    assert_eq!(search_results.valid_todos.len(), 2);
}

#[test]
fn tdignore_always() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_tdignore").to_path_buf(),
        false,
        "*".to_string(),
    )
    .unwrap();

    assert_eq!(search_results.files_searched, 1);
    assert_eq!(search_results.valid_todos.len(), 1);

    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/ignored_tdignore").to_path_buf(),
        true,
        "*".to_string(),
    )
    .unwrap();

    assert_eq!(search_results.files_searched, 1);
    assert_eq!(search_results.valid_todos.len(), 1);
}
