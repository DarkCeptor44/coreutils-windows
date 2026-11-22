use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, prelude::predicate};

#[test]
fn test_which_cli_existing_binary_on_path() {
    let mut cmd = Command::cargo_bin("which").unwrap();
    cmd.arg("cargo")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_which_cli_fails_on_empty_pattern() {
    let mut cmd = Command::cargo_bin("which").unwrap();
    cmd.arg("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty"));
}
