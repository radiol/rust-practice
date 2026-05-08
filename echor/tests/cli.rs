use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello World"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "World"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_omit_newline() -> TestResult {
    run(&["-n", "Hello   World"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_omit_newline() -> TestResult {
    run(&["-n", "Hello", "World"], "tests/expected/hello2.n.txt")
}
