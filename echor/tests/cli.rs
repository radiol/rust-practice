use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello World").assert().success().stdout(expected);
}

#[test]
fn hello2() {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["Hello", "World"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello1_omit_newline() {
    let outfile = "tests/expected/hello1.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["-n", "Hello   World"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello2_omit_newline() {
    let outfile = "tests/expected/hello2.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["-n", "Hello", "World"])
        .assert()
        .success()
        .stdout(expected);
}
