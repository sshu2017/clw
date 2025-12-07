use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_stats_numeric_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("--column")
        .arg("age")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'age' Statistics (Numeric)",
        ))
        .stdout(predicate::str::contains("Count:"))
        .stdout(predicate::str::contains("Mean:"))
        .stdout(predicate::str::contains("Std Dev:"))
        .stdout(predicate::str::contains("Min:"))
        .stdout(predicate::str::contains("Max:"))
        .stdout(predicate::str::contains("Percentiles:"))
        .stdout(predicate::str::contains("1%:"))
        .stdout(predicate::str::contains("25%:"))
        .stdout(predicate::str::contains("50%:"))
        .stdout(predicate::str::contains("75%:"))
        .stdout(predicate::str::contains("99%:"));
}

#[test]
fn test_stats_categorical_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("--column")
        .arg("name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'name' Statistics (Categorical)",
        ))
        .stdout(predicate::str::contains("Total Count:"))
        .stdout(predicate::str::contains("Unique:"))
        .stdout(predicate::str::contains("Top 3 Most Frequent:"));
}

#[test]
fn test_stats_short_flag() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("city")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'city' Statistics (Categorical)",
        ));
}

#[test]
fn test_stats_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("age")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'age' Statistics (Numeric)",
        ));
}

#[test]
fn test_stats_pipe_delimited() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("price")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'price' Statistics (Numeric)",
        ));
}

#[test]
fn test_stats_invalid_column() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("invalid_column")
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
fn test_stats_numeric_values() {
    // Test that numeric statistics are correctly calculated
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("age")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Count:        3"));

    // The ages are 30, 25, 35
    // Mean should be 30.00
    // Min should be 25.00
    // Max should be 35.00
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Mean:         30.00"),
        "Mean should be 30.00"
    );
    assert!(
        stdout.contains("Min:          25.00"),
        "Min should be 25.00"
    );
    assert!(
        stdout.contains("Max:          35.00"),
        "Max should be 35.00"
    );
}

#[test]
fn test_stats_categorical_values() {
    // Test that categorical statistics show the right unique count
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("name")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total Count:  3"))
        .stdout(predicate::str::contains("Unique:       3"));
}

#[test]
fn test_stats_single_column_csv() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("username")
        .arg("tests/fixtures/single_column.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Column 'username' Statistics (Categorical)",
        ));
}

#[test]
fn test_stats_missing_column_flag() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_stats_with_empty_values() {
    // Create a CSV with some empty values
    let temp_csv = "name,age\nAlice,30\nBob,\nCharlie,35\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("age")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("Null/Empty:   1"))
        .stdout(predicate::str::contains(
            "Column 'age' Statistics (Numeric)",
        ));
}

#[test]
fn test_stats_mixed_numeric_categorical() {
    // Create a CSV with mixed values in a column
    let temp_csv = "value\n10\n20\nabc\n30\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success();

    // With 3 numeric and 1 non-numeric, should be treated as numeric
    // (more than 50% are numeric)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Column 'value' Statistics (Numeric)"),
        "Should be treated as numeric"
    );
    assert!(
        stdout.contains("Invalid:      1"),
        "Should show 1 invalid value"
    );
}

#[test]
fn test_stats_mostly_categorical() {
    // Create a CSV where most values are non-numeric
    let temp_csv = "value\nabc\ndef\n10\nghi\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success();

    // With 1 numeric and 3 non-numeric, should be treated as categorical
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Column 'value' Statistics (Categorical)"),
        "Should be treated as categorical"
    );
}

#[test]
fn test_stats_all_empty() {
    // Create a CSV with all empty field values (not empty lines)
    // Empty lines are not parsed as records by the CSV parser
    let temp_csv = "value\n \n \n \n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("Null/Empty:   3"))
        .stdout(predicate::str::contains("No non-empty values to analyze"));
}

#[test]
fn test_stats_single_value() {
    // Create a CSV with a single value
    let temp_csv = "value\n42\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("Count:        1"))
        .stdout(predicate::str::contains("Mean:         42.00"))
        .stdout(predicate::str::contains("Min:          42.00"))
        .stdout(predicate::str::contains("Max:          42.00"));
}

#[test]
fn test_stats_percentiles() {
    // Create a CSV with known values to test percentile calculation
    let temp_csv = "value\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // For values 1-10, median (50%) should be 5.5
    assert!(stdout.contains("50%:        5.50"), "Median should be 5.50");
    assert!(stdout.contains("Min:          1.00"), "Min should be 1.00");
    assert!(
        stdout.contains("Max:          10.00"),
        "Max should be 10.00"
    );
}

#[test]
fn test_stats_top_frequencies() {
    // Create a CSV to test top frequency calculation
    let temp_csv = "city\nNew York\nLos Angeles\nNew York\nChicago\nNew York\nLos Angeles\n";

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("stats")
        .arg("-c")
        .arg("city")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // New York appears 3 times (50%), Los Angeles 2 times (33.3%), Chicago 1 time (16.7%)
    assert!(stdout.contains("New York"), "Should show New York");
    assert!(stdout.contains("3"), "Should show count of 3 for New York");
    assert!(stdout.contains("50.0%"), "Should show 50% for New York");
}
