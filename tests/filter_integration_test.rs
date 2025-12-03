use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_filter_single_value() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("--column")
        .arg("name")
        .arg("--value")
        .arg("Alice")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"));

    // Should have 2 lines (header + 1 matching row)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have header + 1 matching row");
}

#[test]
fn test_filter_multiple_values() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("--column")
        .arg("name")
        .arg("--value")
        .arg("Alice,Bob")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"))
        .stdout(predicate::str::contains("Bob,25,Los Angeles,Designer"));

    // Should have 3 lines (header + 2 matching rows)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3, "Should have header + 2 matching rows");
}

#[test]
fn test_filter_short_flags() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("city")
        .arg("-v")
        .arg("Chicago")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stdout(predicate::str::contains("Charlie,35,Chicago,Manager"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have header + 1 matching row");
}

#[test]
fn test_filter_without_keep_header() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("Alice")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"));

    // Should not have header row (default behavior)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("name,age,city,occupation"),
        "Should not contain header by default"
    );

    // Should have exactly 1 data row (no header)
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 1, "Should have only 1 data row without header");
}

#[test]
fn test_filter_with_keep_header() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("occupation")
        .arg("-v")
        .arg("Designer")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stdout(predicate::str::contains("Bob,25,Los Angeles,Designer"));
}

#[test]
fn test_filter_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("city")
        .arg("-v")
        .arg("New York")
        .arg("--keep-header")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"));
}

#[test]
fn test_filter_pipe_delimited() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("product")
        .arg("-v")
        .arg("Widget")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("id|product|price|category"))
        .stdout(predicate::str::contains("1|Widget|9.99|Tools"));
}

#[test]
fn test_filter_invalid_column() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("invalid_column")
        .arg("-v")
        .arg("test")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Column 'invalid_column' not found",
        ))
        .stderr(predicate::str::contains(
            "Available columns: name, age, city, occupation",
        ));
}

#[test]
fn test_filter_no_matches() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("NonExistent")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"))
        .stderr(predicate::str::contains(
            "No rows matched the filter criteria",
        ))
        .stderr(predicate::str::contains("Column: 'name'"))
        .stderr(predicate::str::contains("'NonExistent'"));

    // Should have only header (no matching rows)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 1, "Should have only header when no matches");
}

#[test]
fn test_filter_with_spaces_in_value_list() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("Alice, Bob")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"))
        .stdout(predicate::str::contains("Bob,25,Los Angeles,Designer"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3, "Should have header + 2 matching rows");
}

#[test]
fn test_filter_numeric_column() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("age")
        .arg("-v")
        .arg("30,35")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"))
        .stdout(predicate::str::contains("Charlie,35,Chicago,Manager"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Bob,25"),
        "Should not contain Bob with age 25"
    );
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3, "Should have header + 2 matching rows");
}

#[test]
fn test_filter_all_rows() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("Alice,Bob,Charlie")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice,30,New York,Engineer"))
        .stdout(predicate::str::contains("Bob,25,Los Angeles,Designer"))
        .stdout(predicate::str::contains("Charlie,35,Chicago,Manager"));

    // Should have 4 lines (header + 3 matching rows)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 4, "Should have header + all 3 rows");
}

#[test]
fn test_filter_single_column_csv() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("username")
        .arg("-v")
        .arg("alice123")
        .arg("--keep-header")
        .arg("tests/fixtures/single_column.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("username"))
        .stdout(predicate::str::contains("alice123"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("bob456"), "Should not contain bob456");
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have header + 1 matching row");
}

#[test]
fn test_filter_case_sensitive() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("alice")
        .arg("--keep-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "No rows matched the filter criteria",
        ));

    // Should have only header (case doesn't match)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Alice,30"),
        "Should be case sensitive - 'alice' != 'Alice'"
    );
    let line_count = stdout.lines().count();
    assert_eq!(
        line_count, 1,
        "Should have only header when case doesn't match"
    );
}

#[test]
fn test_filter_no_matches_without_header() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("-v")
        .arg("NonExistent")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "No rows matched the filter criteria",
        ))
        .stderr(predicate::str::contains("Column: 'name'"));

    // Should have no output at all (default is no header)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(
        line_count, 0,
        "Should have no output when no matches and no --keep-header"
    );
}

#[test]
fn test_filter_missing_column_flag() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-v")
        .arg("Alice")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_filter_missing_value_flag() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("filter")
        .arg("-c")
        .arg("name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
