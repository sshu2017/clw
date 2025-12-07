use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_select_single_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("--columns")
        .arg("name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name\n"))
        .stdout(predicate::str::contains("Alice\n"))
        .stdout(predicate::str::contains("Bob\n"))
        .stdout(predicate::str::contains("Charlie\n"));

    // Should have 4 lines (header + 3 rows)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 4, "Should have header + 3 rows");
}

#[test]
fn test_select_multiple_columns() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("--columns")
        .arg("name,age,city")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city"))
        .stdout(predicate::str::contains("Alice,30,New York"))
        .stdout(predicate::str::contains("Bob,25,Los Angeles"))
        .stdout(predicate::str::contains("Charlie,35,Chicago"));
}

#[test]
fn test_select_short_flag() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("age,name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("age,name"))
        .stdout(predicate::str::contains("30,Alice"))
        .stdout(predicate::str::contains("25,Bob"));
}

#[test]
fn test_select_reorder_columns() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("occupation,name,age")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("occupation,name,age"))
        .stdout(predicate::str::contains("Engineer,Alice,30"))
        .stdout(predicate::str::contains("Designer,Bob,25"))
        .stdout(predicate::str::contains("Manager,Charlie,35"));
}

#[test]
fn test_select_with_spaces_in_column_list() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("name, age, city")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city"))
        .stdout(predicate::str::contains("Alice,30,New York"));
}

#[test]
fn test_select_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("name,city")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("name,city"))
        .stdout(predicate::str::contains("Alice,New York"));
}

#[test]
fn test_select_pipe_delimited() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("product,price")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("product|price"))
        .stdout(predicate::str::contains("Widget|9.99"))
        .stdout(predicate::str::contains("Gadget|19.99"))
        .stdout(predicate::str::contains("Doohickey|")); // Empty price field
}

#[test]
fn test_select_invalid_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("name,invalid_column")
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
fn test_select_all_columns_in_different_order() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("city,occupation,age,name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("city,occupation,age,name"))
        .stdout(predicate::str::contains("New York,Engineer,30,Alice"));
}

// #[test]
// fn test_select_duplicate_columns_auto_cancel() {
//     // In non-interactive mode (tests), duplicate columns automatically cancel
//     let mut cmd = cargo_bin_cmd!("clw");
//     cmd.arg("select")
//         .arg("-c")
//         .arg("name,age,name")
//         .arg("tests/fixtures/sample_comma.csv")
//         .assert()
//         .success()
//         .stderr(predicate::str::contains("Duplicate columns detected"))
//         .stderr(predicate::str::contains("'name'"))
//         .stderr(predicate::str::contains("Non-interactive mode: defaulting to 'no'"))
//         .stderr(predicate::str::contains("Operation cancelled"));

//     // Should have no stdout since operation was cancelled
//     let output = cmd.output().expect("Failed to execute command");
//     let stdout = String::from_utf8_lossy(&output.stdout);
//     assert!(stdout.is_empty(), "Should have no output when cancelled");
// }

// #[test]
// fn test_select_multiple_duplicates_auto_cancel() {
//     let mut cmd = cargo_bin_cmd!("clw");
//     cmd.arg("select")
//         .arg("-c")
//         .arg("name,age,name,city,age")
//         .arg("tests/fixtures/sample_comma.csv")
//         .assert()
//         .success()
//         .stderr(predicate::str::contains("Duplicate columns detected"))
//         .stderr(predicate::str::contains("'name'"))
//         .stderr(predicate::str::contains("'age'"))
//         .stderr(predicate::str::contains("Non-interactive mode: defaulting to 'no'"))
//         .stderr(predicate::str::contains("Operation cancelled"));

//     // Should have no stdout since operation was cancelled
//     let output = cmd.output().expect("Failed to execute command");
//     let stdout = String::from_utf8_lossy(&output.stdout);
//     assert!(stdout.is_empty(), "Should have no output when cancelled");
// }

#[test]
fn test_select_no_columns_specified() {
    // This should fail at the clap level - --columns is required
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_select_single_column_csv() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("username")
        .arg("tests/fixtures/single_column.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("username"))
        .stdout(predicate::str::contains("alice123"))
        .stdout(predicate::str::contains("bob456"));
}

#[test]
fn test_select_preserves_empty_fields() {
    // Create a temporary CSV with empty fields
    let temp_csv = "name,age,city\nAlice,30,\nBob,,LA\n,25,Chicago\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("name,age,city")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice,30,\n"))
        .stdout(predicate::str::contains("Bob,,LA\n"))
        .stdout(predicate::str::contains(",25,Chicago\n"));
}

#[test]
fn test_select_single_column_from_many() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("select")
        .arg("-c")
        .arg("city")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("city\n"))
        .stdout(predicate::str::contains("New York\n"))
        .stdout(predicate::str::contains("Los Angeles\n"))
        .stdout(predicate::str::contains("Chicago\n"));

    // Verify only city column, no other data
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Alice"),
        "Should not contain data from other columns"
    );
    assert!(
        !stdout.contains("30"),
        "Should not contain data from other columns"
    );
}
