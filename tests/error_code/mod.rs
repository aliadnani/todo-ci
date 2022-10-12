use assert_cmd::Command;

#[test]
fn test_error_code_emitted_if_overdue_todos_found() {
    let mut cmd = Command::cargo_bin("todo-ci").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_error_code_can_be_supressed() {
    let mut cmd = Command::cargo_bin("todo-ci").unwrap();

    cmd.args(["--no-error"]).assert().success();
}
