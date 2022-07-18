use std::path::Path;

use chrono::NaiveDate;
use todo_ci::core::{Todo, TodoState};

#[test]
fn overdue_todos() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/simple").to_path_buf(),
        false,
        "*".to_string(),
    )
    .unwrap();

    assert_eq!(search_results.statistics.files_searched, 3);

    // TODO: find a way to mock time
    // Overude TODO on line 3
    assert_eq!(search_results.statistics.overdue_todo_count, 1);
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Overdue) })
            .collect::<Vec<&Todo>>()[0]
            .date
            .unwrap(),
        NaiveDate::from_ymd(1991, 7, 10)
    );
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Overdue) })
            .collect::<Vec<&Todo>>()[0]
            .description,
        "Print something besides \"Hello World!\""
    );
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Overdue) })
            .collect::<Vec<&Todo>>()[0]
            .line_number,
        3
    );
}

#[test]
fn valid_todos() {
    let search_results = todo_ci::core::search(
        Path::new("./tests/resources/simple").to_path_buf(),
        false,
        "*".to_string(),
    )
    .unwrap();

    // Valid TODO on line 4
    assert_eq!(search_results.statistics.valid_todo_count, 1);
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Valid) })
            .collect::<Vec<&Todo>>()[0]
            .file,
        Path::new("./tests/resources/simple/file_with_todos.rs").to_path_buf()
    );
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Valid) })
            .collect::<Vec<&Todo>>()[0]
            .date
            .unwrap(),
        NaiveDate::from_ymd(2221, 7, 10)
    );
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Valid) })
            .collect::<Vec<&Todo>>()[0]
            .description,
        "Print something besides \"Hello World!\" on this line!"
    );
    assert_eq!(
        search_results
            .todos
            .iter()
            .filter(|td| { matches!(td.state, TodoState::Valid) })
            .collect::<Vec<&Todo>>()[0]
            .line_number,
        4
    );
}
