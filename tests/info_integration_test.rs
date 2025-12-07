use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_info_clean_csv() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"))
        .stdout(predicate::str::contains("Rows:"))
        .stdout(predicate::str::contains("Columns:"));

    // Should not have warnings for clean data
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("WARNING"));
}

#[test]
fn test_info_pipe_delimited() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"));
}

#[test]
fn test_info_dirty_csv_with_inconsistent_rows() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/dirty_inconsistent.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"))
        .stdout(predicate::str::contains("Rows:    8"))
        .stdout(predicate::str::contains("Columns: 4"))
        .stdout(predicate::str::contains(
            "WARNING: Inconsistent row lengths detected!",
        ))
        .stdout(predicate::str::contains("Expected 4 fields per row"))
        .stdout(predicate::str::contains("Total inconsistent rows: 4/8"))
        .stdout(predicate::str::contains("Row 3: 3 fields")) // Bob row
        .stdout(predicate::str::contains("Row 4: 5 fields")) // Charlie row
        .stdout(predicate::str::contains("Row 6: 2 fields")) // Eve row
        .stdout(predicate::str::contains("Row 8: 6 fields")); // George row
}

#[test]
fn test_info_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"));
}

#[test]
fn test_info_empty_file() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/empty.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "File is empty or contains no data",
        ));
}

#[test]
fn test_info_single_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/single_column.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"))
        .stdout(predicate::str::contains("Columns: 1"));
}

#[test]
fn test_info_many_inconsistent_rows_truncated() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("info")
        .arg("tests/fixtures/very_dirty.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset Info:"))
        .stdout(predicate::str::contains("Rows:    10"))
        .stdout(predicate::str::contains("Columns: 3"))
        .stdout(predicate::str::contains(
            "WARNING: Inconsistent row lengths detected!",
        ))
        .stdout(predicate::str::contains("... and 1 more inconsistent rows"))
        .stdout(predicate::str::contains("Total inconsistent rows: 6/10"));
}
