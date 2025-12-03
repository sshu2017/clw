use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_header_with_file_argument() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Index"))
        .stdout(predicate::str::contains("ColName"))
        .stdout(predicate::str::contains("0: name"))
        .stdout(predicate::str::contains("1: age"))
        .stdout(predicate::str::contains("2: city"))
        .stdout(predicate::str::contains("3: occupation"));
}

#[test]
fn test_header_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("header")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("Index"))
        .stdout(predicate::str::contains("ColName"))
        .stdout(predicate::str::contains("0: name"))
        .stdout(predicate::str::contains("1: age"))
        .stdout(predicate::str::contains("2: city"))
        .stdout(predicate::str::contains("3: occupation"));
}

#[test]
fn test_header_with_pipe_delimiter() {
    let csv_content = fs::read("tests/fixtures/sample_pipe.csv").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("header")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("Index"))
        .stdout(predicate::str::contains("ColName"))
        .stdout(predicate::str::contains("0: id"))
        .stdout(predicate::str::contains("1: product"))
        .stdout(predicate::str::contains("2: price"))
        .stdout(predicate::str::contains("3: category"));
}

#[test]
fn test_header_empty_file_error() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("header")
        .arg("tests/fixtures/empty.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "File is empty or contains no data",
        ));
}

#[test]
fn test_header_nonexistent_file() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("header")
        .arg("tests/fixtures/does_not_exist.csv")
        .assert()
        .failure();
}
