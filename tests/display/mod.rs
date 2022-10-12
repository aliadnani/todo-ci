use std::path::Path;

use chrono::{FixedOffset, NaiveDate};
use termcolor::{BufferWriter, ColorChoice};
use todo_ci::{
    cli::DisplayMode,
    core::{SearchResult, Todo, TodoState, TodoStatistics},
    display,
};


fn sample_todo_search_result() -> SearchResult {
    SearchResult {
        todos: vec![
            Todo {
                file: Path::new("./file.rs").into(),
                line_number: 3,
                date: Some(NaiveDate::from_ymd(2221, 7, 10)),
                description: String::from("Valid TODO on line 3"),
                state: TodoState::Valid,
            },
            Todo {
                file: Path::new("./file.rs").into(),
                line_number: 5,
                date: Some(NaiveDate::from_ymd(1990, 7, 10)),
                description: String::from("Expired TODO on line 5"),
                state: TodoState::Overdue,
            },
            Todo {
                file: Path::new("./file.rs").into(),
                line_number: 7,
                date: None,
                description: String::from("XX is not a valid date."),
                state: TodoState::Malformed,
            },
        ],
        statistics: TodoStatistics {
            files_searched: 1,
            valid_todo_count: 1,
            overdue_todo_count: 1,
            malformed_todo_count: 1,
        },
    }
}

#[test]
fn test_default_terminal_output() {
    // ColorChoice::Never removes the noisy ANSI escape codes making for easier assertions
    let bufwtr = BufferWriter::stdout(ColorChoice::Never);
    let mut buffer = bufwtr.buffer();

    display::print(
        &mut buffer,
        DisplayMode::Default,
        &sample_todo_search_result(),
        &FixedOffset::west(0),
    );

    let output = String::from_utf8_lossy(buffer.as_slice());

    // Assertions are intentionally not very specific -
    // otherwise they would just break often even on minor changes to the printer 
    assert!(output.contains("Valid TODO on line 3"));
    assert!(output.contains("Expired TODO on line 5"));
    assert!(output.contains("XX is not a valid date."));

    assert!(output.contains("Searched 1 file(s):"));
    assert!(output.contains("2 todo(s) found of which 1 is/are overdue"));
}

#[test]
fn test_overdue_only_terminal_output() {
    // ColorChoice::Never removes the noisy ANSI escape codes making for easier assertions
    let bufwtr = BufferWriter::stdout(ColorChoice::Never);
    let mut buffer = bufwtr.buffer();

    display::print(
        &mut buffer,
        DisplayMode::OverdueOnly,
        &sample_todo_search_result(),
        &FixedOffset::west(0),
    );

    let output = String::from_utf8_lossy(buffer.as_slice());

    // Assertions are intentionally not very specific -
    // otherwise they would just break often even on minor changes to the printer 
    assert!(!output.contains("Valid TODO on line 3"));
    assert!(output.contains("Expired TODO on line 5"));
    assert!(!output.contains("XX is not a valid date."));

    assert!(output.contains("Searched 1 file(s):"));
    assert!(output.contains("2 todo(s) found of which 1 is/are overdue"));
}

#[test]
fn test_concise_terminal_output() {
    // ColorChoice::Never removes the noisy ANSI escape codes making for easier assertions
    let bufwtr = BufferWriter::stdout(ColorChoice::Never);
    let mut buffer = bufwtr.buffer();

    display::print(
        &mut buffer,
        DisplayMode::Concise,
        &sample_todo_search_result(),
        &FixedOffset::west(0),
    );

    let output = String::from_utf8_lossy(buffer.as_slice());

    // Assertions are intentionally not very specific -
    // otherwise they would just break often even on minor changes to the printer 
    assert!(!output.contains("Valid TODO on line 3"));
    assert!(!output.contains("Expired TODO on line 5"));
    assert!(!output.contains("XX is not a valid date."));

    assert!(output.contains("Searched 1 file(s):"));
    assert!(output.contains("2 todo(s) found of which 1 is/are overdue"));
}
