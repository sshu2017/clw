use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_stack_basic_comma_delimited() {
    // Create two temporary CSV files with matching headers
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age,city").unwrap();
    writeln!(file1, "Alice,30,New York").unwrap();
    writeln!(file1, "Bob,25,Los Angeles").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,age,city").unwrap();
    writeln!(file2, "Charlie,35,Chicago").unwrap();
    writeln!(file2, "Dave,28,Boston").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have exactly one header row
    let header_count = stdout
        .lines()
        .filter(|line| line.starts_with("name,age,city"))
        .count();
    assert_eq!(header_count, 1, "Should have exactly one header row");

    // Should have all data rows (4 total: 2 from file1 + 2 from file2)
    assert!(stdout.contains("Alice,30,New York"), "Should contain Alice");
    assert!(stdout.contains("Bob,25,Los Angeles"), "Should contain Bob");
    assert!(
        stdout.contains("Charlie,35,Chicago"),
        "Should contain Charlie"
    );
    assert!(stdout.contains("Dave,28,Boston"), "Should contain Dave");

    // Total lines should be 5 (1 header + 4 data rows)
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(
        line_count, 5,
        "Should have 5 total lines (1 header + 4 data)"
    );
}

#[test]
fn test_stack_pipe_delimited() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id|product|price").unwrap();
    writeln!(file1, "1|Widget|9.99").unwrap();
    writeln!(file1, "2|Gadget|19.99").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "id|product|price").unwrap();
    writeln!(file2, "3|Doohickey|14.99").unwrap();
    writeln!(file2, "4|Thingamajig|24.99").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have exactly one header row
    let header_count = stdout
        .lines()
        .filter(|line| line.starts_with("id|product|price"))
        .count();
    assert_eq!(header_count, 1, "Should have exactly one header row");

    // Should have all data rows
    assert!(stdout.contains("1|Widget|9.99"), "Should contain Widget");
    assert!(stdout.contains("2|Gadget|19.99"), "Should contain Gadget");
    assert!(
        stdout.contains("3|Doohickey|14.99"),
        "Should contain Doohickey"
    );
    assert!(
        stdout.contains("4|Thingamajig|24.99"),
        "Should contain Thingamajig"
    );
}

#[test]
fn test_stack_different_delimiters() {
    // File 1 uses comma
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    // File 2 uses pipe
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name|age").unwrap();
    writeln!(file2, "Bob|25").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Files have different delimiters"));
}

#[test]
fn test_stack_different_column_count() {
    // File 1 has 3 columns
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age,city").unwrap();
    writeln!(file1, "Alice,30,New York").unwrap();

    // File 2 has 2 columns
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,age").unwrap();
    writeln!(file2, "Bob,25").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Files have different number of columns",
        ));
}

#[test]
fn test_stack_different_header_names() {
    // File 1 has headers: name, age
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    // File 2 has headers: name, city (different second column)
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,city").unwrap();
    writeln!(file2, "Bob,Boston").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Headers don't match at position"));
}

#[test]
fn test_stack_single_row_files() {
    // Both files have only header and one data row
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id,value").unwrap();
    writeln!(file1, "1,A").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "id,value").unwrap();
    writeln!(file2, "2,B").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have 1 header + 2 data rows = 3 lines
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 3, "Should have 3 total lines");

    assert!(stdout.contains("1,A"), "Should contain first row");
    assert!(stdout.contains("2,B"), "Should contain second row");
}

#[test]
fn test_stack_file_not_found() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg("/nonexistent/file.csv")
        .assert()
        .failure();
}

#[test]
fn test_stack_with_empty_values() {
    // Test files with empty field values
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age,city").unwrap();
    writeln!(file1, "Alice,30,").unwrap();
    writeln!(file1, "Bob,,Boston").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,age,city").unwrap();
    writeln!(file2, "Charlie,35,Chicago").unwrap();
    writeln!(file2, ",,").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have all rows including those with empty values
    assert!(
        stdout.contains("Alice,30,"),
        "Should contain Alice with empty city"
    );
    assert!(
        stdout.contains("Bob,,Boston"),
        "Should contain Bob with empty age"
    );
    assert!(
        stdout.contains("Charlie,35,Chicago"),
        "Should contain Charlie"
    );
    assert!(
        stdout.contains(",,"),
        "Should contain row with all empty values"
    );
}

#[test]
fn test_stack_preserves_order() {
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "id,name").unwrap();
    writeln!(file1, "1,First").unwrap();
    writeln!(file1, "2,Second").unwrap();
    writeln!(file1, "3,Third").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "id,name").unwrap();
    writeln!(file2, "4,Fourth").unwrap();
    writeln!(file2, "5,Fifth").unwrap();
    writeln!(file2, "6,Sixth").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that order is preserved (file1 rows come before file2 rows)
    let lines: Vec<&str> = stdout.lines().collect();

    // Find positions of data rows
    let first_pos = lines
        .iter()
        .position(|&line| line.contains("1,First"))
        .unwrap();
    let second_pos = lines
        .iter()
        .position(|&line| line.contains("2,Second"))
        .unwrap();
    let third_pos = lines
        .iter()
        .position(|&line| line.contains("3,Third"))
        .unwrap();
    let fourth_pos = lines
        .iter()
        .position(|&line| line.contains("4,Fourth"))
        .unwrap();
    let fifth_pos = lines
        .iter()
        .position(|&line| line.contains("5,Fifth"))
        .unwrap();
    let sixth_pos = lines
        .iter()
        .position(|&line| line.contains("6,Sixth"))
        .unwrap();

    // Verify order
    assert!(first_pos < second_pos, "First should come before Second");
    assert!(second_pos < third_pos, "Second should come before Third");
    assert!(third_pos < fourth_pos, "Third should come before Fourth");
    assert!(fourth_pos < fifth_pos, "Fourth should come before Fifth");
    assert!(fifth_pos < sixth_pos, "Fifth should come before Sixth");
}

#[test]
fn test_stack_many_columns() {
    // Test with a CSV that has many columns
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "col1,col2,col3,col4,col5,col6,col7,col8,col9,col10").unwrap();
    writeln!(file1, "a1,a2,a3,a4,a5,a6,a7,a8,a9,a10").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "col1,col2,col3,col4,col5,col6,col7,col8,col9,col10").unwrap();
    writeln!(file2, "b1,b2,b3,b4,b5,b6,b7,b8,b9,b10").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("a1,a2,a3,a4,a5,a6,a7,a8,a9,a10"),
        "Should contain file1 data"
    );
    assert!(
        stdout.contains("b1,b2,b3,b4,b5,b6,b7,b8,b9,b10"),
        "Should contain file2 data"
    );
}

#[test]
fn test_stack_header_case_sensitive() {
    // Headers differ only in case - should fail
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "Name,Age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,age").unwrap();
    writeln!(file2, "Bob,25").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Headers don't match at position"));
}

#[test]
fn test_stack_with_quoted_fields() {
    // Test with CSV files containing quoted fields
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,description").unwrap();
    writeln!(file1, "Alice,\"Software Engineer\"").unwrap();

    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,description").unwrap();
    writeln!(file2, "Bob,\"Product Manager\"").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // The output should contain the data (quotes may or may not be preserved depending on csv crate behavior)
    assert!(
        stdout.contains("Alice") && stdout.contains("Software Engineer"),
        "Should contain Alice's data"
    );
    assert!(
        stdout.contains("Bob") && stdout.contains("Product Manager"),
        "Should contain Bob's data"
    );
}

#[test]
fn test_stack_header_only_file() {
    // File 1 has header and data
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "name,age").unwrap();
    writeln!(file1, "Alice,30").unwrap();

    // File 2 has only header, no data rows
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "name,age").unwrap();

    let mut cmd = Command::cargo_bin("clw").unwrap();
    cmd.arg("stack")
        .arg(file1.path())
        .arg(file2.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have header and Alice's row
    assert!(stdout.contains("name,age"), "Should have header");
    assert!(stdout.contains("Alice,30"), "Should have Alice's data");

    // Should have exactly 2 lines (header + 1 data row)
    let line_count = stdout.lines().filter(|line| !line.is_empty()).count();
    assert_eq!(line_count, 2, "Should have 2 total lines");
}
