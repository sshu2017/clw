use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_freq_basic() {
    // Test basic frequency counting (default: sorted by frequency high to low)
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("--column")
        .arg("city")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("New York"))
        .stdout(predicate::str::contains("Los Angeles"))
        .stdout(predicate::str::contains("Chicago"));

    // Each city appears once, so all should have count of 1
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1"), "Should show count of 1");
}

#[test]
fn test_freq_with_duplicates() {
    // Create CSV with duplicate values
    let temp_csv = "city\nNew York\nLos Angeles\nNew York\nChicago\nNew York\nLos Angeles\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("city")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // New York appears 3 times, Los Angeles 2 times, Chicago 1 time
    // Default sort is by frequency (high to low), so New York should be first
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(
        lines[1].contains("New York") && lines[1].contains("3"),
        "First data line should be New York with 3"
    );
    assert!(
        lines[2].contains("Los Angeles") && lines[2].contains("2"),
        "Second data line should be Los Angeles with 2"
    );
    assert!(
        lines[3].contains("Chicago") && lines[3].contains("1"),
        "Third data line should be Chicago with 1"
    );
}

#[test]
fn test_freq_with_plot() {
    let temp_csv = "city\nNew York\nLos Angeles\nNew York\nChicago\nNew York\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("city")
        .arg("--plot")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("▪")); // Should contain bar characters

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show values, counts, and bars
    assert!(stdout.contains("New York"), "Should show New York");
    assert!(stdout.contains("3"), "Should show count 3");
    assert!(stdout.contains("▪"), "Should show bar plot");
}

#[test]
fn test_freq_short_flags() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("occupation")
        .arg("-p")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("▪"));
}

#[test]
fn test_freq_sort_index_alphabetic() {
    let temp_csv = "name\nCharlie\nAlice\nBob\nAlice\nCharlie\nCharlie\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("name")
        .arg("--sort-index")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be sorted alphabetically: Alice, Bob, Charlie
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(
        lines[1].contains("Alice"),
        "First data line should be Alice"
    );
    assert!(lines[2].contains("Bob"), "Second data line should be Bob");
    assert!(
        lines[3].contains("Charlie"),
        "Third data line should be Charlie"
    );
}

#[test]
fn test_freq_sort_index_numeric() {
    let temp_csv = "value\n30\n10\n25\n30\n10\n10\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("value")
        .arg("--sort-index")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be sorted numerically: 10, 25, 30
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines[1].starts_with("10"), "First data line should be 10");
    assert!(lines[2].starts_with("25"), "Second data line should be 25");
    assert!(lines[3].starts_with("30"), "Third data line should be 30");
}

#[test]
fn test_freq_sort_value() {
    // Test default sorting (by frequency, high to low)
    let temp_csv = "name\nAlice\nBob\nAlice\nCharlie\nAlice\nBob\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("name")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be sorted by frequency: Alice (3), Bob (2), Charlie (1)
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(
        lines[1].contains("Alice") && lines[1].contains("3"),
        "First data line should be Alice with 3"
    );
    assert!(
        lines[2].contains("Bob") && lines[2].contains("2"),
        "Second data line should be Bob with 2"
    );
    assert!(
        lines[3].contains("Charlie") && lines[3].contains("1"),
        "Third data line should be Charlie with 1"
    );
}

#[test]
fn test_freq_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("occupation")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("Engineer"))
        .stdout(predicate::str::contains("Designer"))
        .stdout(predicate::str::contains("Manager"));
}

#[test]
fn test_freq_pipe_delimited() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("category")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Tools"))
        .stdout(predicate::str::contains("Electronics"))
        .stdout(predicate::str::contains("Home"));
}

#[test]
fn test_freq_invalid_column() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
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
fn test_freq_single_value() {
    let temp_csv = "value\n42\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("42"))
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_freq_all_same_value() {
    let temp_csv = "name\nAlice\nAlice\nAlice\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("name")
        .write_stdin(temp_csv)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_freq_with_plot_alignment() {
    let temp_csv = "city\nNew York\nLA\nNew York\nChicago\nNew York\nLA\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("city")
        .arg("--plot")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that all three values appear with bars
    assert!(
        stdout.contains("New York") && stdout.contains("3"),
        "Should show New York with count 3"
    );
    assert!(
        stdout.contains("LA") && stdout.contains("2"),
        "Should show LA with count 2"
    );
    assert!(
        stdout.contains("Chicago") && stdout.contains("1"),
        "Should show Chicago with count 1"
    );

    // Data lines (not headers/separators) should contain bar character
    // Check that the bar character appears in lines with city names
    for line in stdout.lines() {
        if line.contains("New York") || line.contains("LA") || line.contains("Chicago") {
            assert!(
                line.contains("▪"),
                "Data line should contain a bar: {}",
                line
            );
        }
    }
}

#[test]
fn test_freq_default_sort_is_by_frequency() {
    // Test that default behavior (no sort flags) sorts by frequency
    let temp_csv = "value\nA\nB\nA\nC\nA\nB\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be sorted by frequency: A (3), B (2), C (1)
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(
        lines[1].starts_with("A"),
        "First data line should be A (highest frequency)"
    );
    assert!(lines[2].starts_with("B"), "Second data line should be B");
    assert!(
        lines[3].starts_with("C"),
        "Third data line should be C (lowest frequency)"
    );
}

#[test]
fn test_freq_missing_column_flag() {
    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_freq_with_empty_values() {
    // Test that empty field values are counted
    // Using multi-column CSV where one column has empty values
    let temp_csv = "name,value\nAlice,A\nBob,\nCharlie,B\nDave,A\nEve,\nFrank,A\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("value")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // A appears 3 times, empty appears 2 times, B appears 1 time
    assert!(
        stdout.contains("A") && stdout.contains("3"),
        "A should appear 3 times"
    );
    assert!(
        stdout.contains("B") && stdout.contains("1"),
        "B should appear 1 time"
    );

    // Empty values should be counted too (3 data rows + 1 header = 4 lines)
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines.len(),
        4,
        "Should have 3 unique values including empty plus header"
    );
}

#[test]
fn test_freq_numeric_sorting() {
    // Test that numeric sorting works correctly (not lexicographic)
    let temp_csv = "number\n100\n20\n3\n100\n20\n100\n";

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("freq")
        .arg("-c")
        .arg("number")
        .arg("--sort-index")
        .write_stdin(temp_csv)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be sorted numerically: 3, 20, 100 (not lexicographically: 100, 20, 3)
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines[1].starts_with("3"), "First data line should be 3");
    assert!(lines[2].starts_with("20"), "Second data line should be 20");
    assert!(lines[3].starts_with("100"), "Third data line should be 100");
}
