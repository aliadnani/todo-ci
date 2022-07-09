use std::path::Path;

use chrono::NaiveDate;

#[test]
fn baseline_test() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/base").to_path_buf(),
        false,
        "*".to_string(),
    )
    .unwrap();

    assert_eq!(search_results.files_searched, 3);

    // TODO: find a way to mock time
    // Overude TODO on line 3
    assert_eq!(search_results.overdue_todos.len(), 1);
    assert_eq!(
        search_results.overdue_todos[0].date,
        NaiveDate::from_ymd(1991, 7, 10)
    );
    assert_eq!(
        search_results.overdue_todos[0].description,
        "Print something besides \"Hello World!\""
    );
    assert_eq!(search_results.overdue_todos[0].line_number, 3);
    assert_eq!(
        search_results.valid_todos[0].file,
        Path::new("./tests/resources/base/file_with_todos.rs").to_path_buf()
    );

    // Valid TODO on line 4
    assert_eq!(search_results.valid_todos.len(), 1);
    assert_eq!(
        search_results.valid_todos[0].date,
        NaiveDate::from_ymd(2221, 7, 10)
    );
    assert_eq!(
        search_results.valid_todos[0].description,
        "Print something besides \"Hello World!\" on this line!"
    );
    assert_eq!(search_results.valid_todos[0].line_number, 4);
    assert_eq!(
        search_results.valid_todos[0].file,
        Path::new("./tests/resources/base/file_with_todos.rs").to_path_buf()
    );
}
