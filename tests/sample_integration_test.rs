use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_sample_with_row_count() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"));

    // Verify we get exactly 3 lines (header + 2 data rows)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3, "Should have header + 2 rows");
}

#[test]
fn test_sample_short_flags() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("-r")
        .arg("1")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have header + 1 row");
}

#[test]
fn test_sample_with_piped_input() {
    let csv_content = fs::read("tests/fixtures/sample_comma.csv").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("1")
        .write_stdin(csv_content)
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"));
}

#[test]
fn test_sample_more_rows_than_available() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("100")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Warning: Requested 100 rows but CSV only has 3 rows",
        ));

    // Should still output all rows
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 4, "Should have header + all 3 rows");
}

#[test]
fn test_sample_pipe_delimited() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("1")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("id|product|price|category"));
}

#[test]
fn test_sample_missing_rows_argument() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_sample_zero_rows() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("0")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success();

    // Should get only header
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 1, "Should have only header");
}

#[test]
fn test_sample_with_no_header_flag() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("--no-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success();

    // Should not have header row
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("name,age,city,occupation"),
        "Should not contain header"
    );

    // Should have exactly 2 data rows (no header)
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have only 2 data rows without header");
}

#[test]
fn test_sample_with_header_default() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("1")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"));

    // Should have header + 1 data row
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 2, "Should have header + 1 data row");
}

#[test]
fn test_sample_no_header_pipe_delimited() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("1")
        .arg("--no-header")
        .arg("tests/fixtures/sample_pipe.csv")
        .assert()
        .success();

    // Should not have header
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("id|product|price|category"),
        "Should not contain header"
    );

    // Should have exactly 1 data row
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 1, "Should have only 1 data row");
}

#[test]
fn test_sample_no_header_zero_rows() {
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("0")
        .arg("--no-header")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success();

    // Should have no output at all
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 0, "Should have no output");
}

#[test]
fn test_sample_with_seed_reproducibility() {
    // Run sample twice with the same seed and verify we get identical output
    let mut cmd1 = cargo_bin_cmd!("clw");
    let output1 = cmd1
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("--seed")
        .arg("42")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let mut cmd2 = cargo_bin_cmd!("clw");
    let output2 = cmd2
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("--seed")
        .arg("42")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    assert_eq!(
        stdout1, stdout2,
        "Same seed should produce identical output"
    );
}

#[test]
fn test_sample_with_different_seeds() {
    // Run sample with different seeds - output should likely be different
    // (though there's a tiny chance they could be the same by random chance)
    let mut cmd1 = cargo_bin_cmd!("clw");
    let output1 = cmd1
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("--seed")
        .arg("42")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let mut cmd2 = cargo_bin_cmd!("clw");
    let output2 = cmd2
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("--seed")
        .arg("123")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    // Both should succeed and have the same structure (header + 2 rows)
    assert_eq!(stdout1.lines().count(), 3, "Should have header + 2 rows");
    assert_eq!(stdout2.lines().count(), 3, "Should have header + 2 rows");

    // Both should have the same header
    assert_eq!(
        stdout1.lines().next(),
        stdout2.lines().next(),
        "Headers should match"
    );
}

#[test]
fn test_sample_with_default_seed() {
    // Test that default seed (67) works
    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("tests/fixtures/sample_comma.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city,occupation"));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    assert_eq!(line_count, 3, "Should have header + 2 rows");
}

#[test]
fn test_sample_default_seed_reproducibility() {
    // Verify that default seed (67) produces reproducible results
    let mut cmd1 = cargo_bin_cmd!("clw");
    let output1 = cmd1
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let mut cmd2 = cargo_bin_cmd!("clw");
    let output2 = cmd2
        .arg("sample")
        .arg("--rows")
        .arg("2")
        .arg("tests/fixtures/sample_comma.csv")
        .output()
        .expect("Failed to execute command");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    assert_eq!(
        stdout1, stdout2,
        "Default seed should produce identical output"
    );
}
