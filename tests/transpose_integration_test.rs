use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_transpose_basic() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "name,age,city").unwrap();
    writeln!(file, "Alice,30,New York").unwrap();
    writeln!(file, "Bob,25,Los Angeles").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // First row should be names
    assert!(
        stdout.contains("name,Alice,Bob"),
        "First row should contain names"
    );
    // Second row should be ages
    assert!(
        stdout.contains("age,30,25"),
        "Second row should contain ages"
    );
    // Third row should be cities
    assert!(
        stdout.contains("city,New York,Los Angeles"),
        "Third row should contain cities"
    );
}

#[test]
fn test_transpose_pipe_delimited() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id|product|price").unwrap();
    writeln!(file, "1|Widget|9.99").unwrap();
    writeln!(file, "2|Gadget|19.99").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("id|1|2"),
        "Should transpose with pipe delimiter"
    );
    assert!(
        stdout.contains("product|Widget|Gadget"),
        "Should preserve pipe delimiter"
    );
    assert!(
        stdout.contains("price|9.99|19.99"),
        "Should handle numeric values"
    );
}

#[test]
fn test_transpose_single_row() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "col1,col2,col3").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have 3 rows (one for each column)
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "Should have 3 rows");
    assert_eq!(lines[0], "col1", "First row should be col1");
    assert_eq!(lines[1], "col2", "Second row should be col2");
    assert_eq!(lines[2], "col3", "Third row should be col3");
}

#[test]
fn test_transpose_single_column() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "value").unwrap();
    writeln!(file, "A").unwrap();
    writeln!(file, "B").unwrap();
    writeln!(file, "C").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should become a single row with all values
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 1, "Should have 1 row after transpose");
    assert_eq!(lines[0], "value,A,B,C", "Should combine all values");
}

#[test]
fn test_transpose_with_empty_values() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c").unwrap();
    writeln!(file, "1,,3").unwrap();
    writeln!(file, ",2,").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that empty values are preserved
    assert!(stdout.contains("a,1,"), "Should preserve empty values");
    assert!(stdout.contains("b,,2"), "Should preserve empty values");
    assert!(stdout.contains("c,3,"), "Should preserve empty values");
}

#[test]
fn test_transpose_piped_input() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "x,y").unwrap();
    writeln!(file, "1,2").unwrap();

    let file_content = std::fs::read_to_string(file.path()).unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose")
        .write_stdin(file_content)
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("x,1"), "Should handle piped input");
    assert!(stdout.contains("y,2"), "Should handle piped input");
}

#[test]
fn test_transpose_jagged_rows() {
    // Test rows with different numbers of columns
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c").unwrap();
    writeln!(file, "1,2").unwrap();
    writeln!(file, "3,4,5,6").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should pad with empty values
    assert!(stdout.contains("a,1,3"), "First column values");
    assert!(stdout.contains("b,2,4"), "Second column values");
    assert!(stdout.contains("c,,5"), "Third column with padding");
    assert!(stdout.contains(",,6"), "Fourth column with padding");
}

#[test]
fn test_transpose_with_quoted_fields() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "name,description").unwrap();
    writeln!(file, "Alice,\"Software Engineer\"").unwrap();
    writeln!(file, "Bob,\"Product Manager\"").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Alice") && stdout.contains("Bob"),
        "Should contain names"
    );
    assert!(
        stdout.contains("Software Engineer") || stdout.contains("Product Manager"),
        "Should contain descriptions"
    );
}

#[test]
fn test_transpose_empty_file() {
    let file = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose")
        .arg(file.path())
        .assert()
        .failure() // Empty files cause an error from delimiter detection
        .stderr(predicate::str::contains("File is empty"));
}

#[test]
fn test_transpose_large_matrix() {
    let mut file = NamedTempFile::new().unwrap();

    // Create a 5x5 matrix
    for i in 0..5 {
        writeln!(
            file,
            "{},{},{},{},{}",
            i * 5,
            i * 5 + 1,
            i * 5 + 2,
            i * 5 + 3,
            i * 5 + 4
        )
        .unwrap();
    }

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("transpose").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check some transposed values
    assert!(
        stdout.contains("0,5,10,15,20"),
        "First column should be 0,5,10,15,20"
    );
    assert!(
        stdout.contains("4,9,14,19,24"),
        "Last column should be 4,9,14,19,24"
    );
}
