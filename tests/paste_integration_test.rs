use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_paste_basic_comma_delimited() {
    // Create two CSV files to paste side by side
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();
    writeln!(file1, "Bob,25").unwrap();
    writeln!(file1, "Charlie,35").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "city,occupation").unwrap();
    writeln!(file2, "New York,Engineer").unwrap();
    writeln!(file2, "Los Angeles,Designer").unwrap();
    writeln!(file2, "Chicago,Manager").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check combined header
    assert!(
        stdout.contains("name,age,city,occupation"),
        "Should have combined headers"
    );

    // Check combined data rows
    assert!(
        stdout.contains("Alice,30,New York,Engineer"),
        "Should have Alice's combined row"
    );
    assert!(
        stdout.contains("Bob,25,Los Angeles,Designer"),
        "Should have Bob's combined row"
    );
    assert!(
        stdout.contains("Charlie,35,Chicago,Manager"),
        "Should have Charlie's combined row"
    );

    // Should have 4 lines total (1 header + 3 data rows)
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 4, "Should have 4 total lines");
}

#[test]
fn test_paste_pipe_delimited() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id|product").unwrap();
    writeln!(file1, "1|Widget").unwrap();
    writeln!(file1, "2|Gadget").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "price|category").unwrap();
    writeln!(file2, "9.99|Tools").unwrap();
    writeln!(file2, "19.99|Electronics").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check combined header with pipe delimiter
    assert!(
        stdout.contains("id|product|price|category"),
        "Should have combined headers with pipe delimiter"
    );

    // Check combined data rows
    assert!(
        stdout.contains("1|Widget|9.99|Tools"),
        "Should have first combined row"
    );
    assert!(
        stdout.contains("2|Gadget|19.99|Electronics"),
        "Should have second combined row"
    );
}

#[test]
fn test_paste_different_delimiters() {
    // File 1 uses comma
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    // File 2 uses pipe
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "city|occupation").unwrap();
    writeln!(file2, "New York|Engineer").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Files have different delimiters"));
}

#[test]
fn test_paste_different_row_counts() {
    // File 1 has 3 data rows
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name").unwrap();
    writeln!(file1, "Alice").unwrap();
    writeln!(file1, "Bob").unwrap();
    writeln!(file1, "Charlie").unwrap();

    // File 2 has 2 data rows
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "city").unwrap();
    writeln!(file2, "New York").unwrap();
    writeln!(file2, "Los Angeles").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("has more rows than"));
}

#[test]
fn test_paste_single_row() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name").unwrap();
    writeln!(file1, "Alice").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "age").unwrap();
    writeln!(file2, "30").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("name,age"), "Should have combined header");
    assert!(stdout.contains("Alice,30"), "Should have combined data row");

    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 2, "Should have 2 lines (header + 1 data row)");
}

#[test]
fn test_paste_empty_values() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,").unwrap();
    writeln!(file1, ",25").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "city,occupation").unwrap();
    writeln!(file2, ",Engineer").unwrap();
    writeln!(file2, "Boston,").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that empty values are preserved
    assert!(
        stdout.contains("Alice,,,Engineer"),
        "Should preserve empty values in first row"
    );
    assert!(
        stdout.contains(",25,Boston,"),
        "Should preserve empty values in second row"
    );
}

#[test]
fn test_paste_single_column_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "col1").unwrap();
    writeln!(file1, "A").unwrap();
    writeln!(file1, "B").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "col2").unwrap();
    writeln!(file2, "X").unwrap();
    writeln!(file2, "Y").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("col1,col2"), "Should have combined header");
    assert!(stdout.contains("A,X"), "Should have first combined row");
    assert!(stdout.contains("B,Y"), "Should have second combined row");
}

#[test]
fn test_paste_many_columns() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "a,b,c").unwrap();
    writeln!(file1, "1,2,3").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "d,e,f").unwrap();
    writeln!(file2, "4,5,6").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("a,b,c,d,e,f"),
        "Should have all 6 columns in header"
    );
    assert!(
        stdout.contains("1,2,3,4,5,6"),
        "Should have all 6 values in data row"
    );
}

#[test]
fn test_paste_preserves_order() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id").unwrap();
    writeln!(file1, "1").unwrap();
    writeln!(file1, "2").unwrap();
    writeln!(file1, "3").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "value").unwrap();
    writeln!(file2, "A").unwrap();
    writeln!(file2, "B").unwrap();
    writeln!(file2, "C").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<&str> = stdout.lines().collect();

    // Check that rows are in correct order
    assert!(lines[1].contains("1,A"), "First row should be 1,A");
    assert!(lines[2].contains("2,B"), "Second row should be 2,B");
    assert!(lines[3].contains("3,C"), "Third row should be 3,C");
}

#[test]
fn test_paste_with_quoted_fields() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name").unwrap();
    writeln!(file1, "\"Smith, John\"").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "role").unwrap();
    writeln!(file2, "\"Senior Engineer\"").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // The CSV should contain the data (quotes may or may not be preserved)
    assert!(
        stdout.contains("Smith") && stdout.contains("John"),
        "Should contain name"
    );
    assert!(stdout.contains("Senior Engineer"), "Should contain role");
}

#[test]
fn test_paste_file_not_found() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name").unwrap();
    writeln!(file1, "Alice").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg("/nonexistent/file.csv")
        .assert()
        .failure();
}

#[test]
fn test_paste_header_only_files() {
    // Both files have only headers, no data rows
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "col1,col2").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "col3,col4").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("col1,col2,col3,col4"),
        "Should have combined header"
    );

    // Should have only 1 line (just the header)
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 1, "Should have only 1 line (header)");
}

#[test]
fn test_paste_duplicate_column_names() {
    // Test that duplicate column names are allowed (they just get combined)
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id,value").unwrap();
    writeln!(file1, "1,A").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "id,value").unwrap();
    writeln!(file2, "2,B").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have duplicate column names in header
    assert!(
        stdout.contains("id,value,id,value"),
        "Should have duplicate column names"
    );
    assert!(stdout.contains("1,A,2,B"), "Should have combined data");
}

#[test]
fn test_paste_large_number_of_rows() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "col1").unwrap();
    for i in 1..=100 {
        writeln!(file1, "{}", i).unwrap();
    }

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "col2").unwrap();
    for i in 101..=200 {
        writeln!(file2, "{}", i).unwrap();
    }

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("paste")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have 101 lines (1 header + 100 data rows)
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 101, "Should have 101 lines");

    // Check first and last data rows
    assert!(stdout.contains("1,101"), "Should have first combined row");
    assert!(stdout.contains("100,200"), "Should have last combined row");
}
