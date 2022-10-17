use assert_cmd::Command;

#[test]
fn test_error_code_emitted_if_overdue_todos_found() {
    let mut cmd = Command::cargo_bin("todo-ci").unwrap();
    cmd.current_dir("./tests/error_code/fixtures")
        .assert()
        .stdout(predicates::str::contains("error_code_file_with_todos.rs:3"))
        .stdout(predicates::str::contains("error_code_file_with_todos.rs:4"))
        .stdout(predicates::str::contains("2 todo(s) found"))
        .failure();
}

#[test]
fn test_error_code_can_be_supressed() {
    let mut cmd = Command::cargo_bin("todo-ci").unwrap();
    
    cmd.args(["--no-error"])
        .current_dir("./tests/error_code/fixtures")
        .assert()
        .stdout(predicates::str::contains("error_code_file_with_todos.rs:3"))
        .stdout(predicates::str::contains("error_code_file_with_todos.rs:4"))
        .stdout(predicates::str::contains("2 todo(s) found"))
        .success();
}
