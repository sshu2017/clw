use assert_cmd::cargo::cargo_bin_cmd;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_peek_basic() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "name,age,city").unwrap();
    writeln!(file, "Alice,30,New York").unwrap();
    writeln!(file, "Bob,25,Los Angeles").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for headers
    assert!(stdout.contains("name"), "Should contain name header");
    assert!(stdout.contains("age"), "Should contain age header");
    assert!(stdout.contains("city"), "Should contain city header");

    // Check for data
    assert!(stdout.contains("Alice"), "Should contain Alice");
    assert!(stdout.contains("Bob"), "Should contain Bob");
    assert!(stdout.contains("30"), "Should contain age 30");
    assert!(stdout.contains("New York"), "Should contain New York");
}

#[test]
fn test_peek_with_max_rows() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id,value").unwrap();
    for i in 1..=10 {
        writeln!(file, "{},value{}", i, i).unwrap();
    }

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek")
        .arg("--number-rows")
        .arg("5")
        .arg(file.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show first 5 rows only
    assert!(stdout.contains("value1"), "Should contain first row");
    assert!(stdout.contains("value5"), "Should contain fifth row");
    assert!(!stdout.contains("value6"), "Should not contain sixth row");
    assert!(!stdout.contains("value10"), "Should not contain tenth row");
}

#[test]
fn test_peek_pipe_delimited() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id|product|price").unwrap();
    writeln!(file, "1|Widget|9.99").unwrap();
    writeln!(file, "2|Gadget|19.99").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Widget"), "Should contain Widget");
    assert!(stdout.contains("Gadget"), "Should contain Gadget");
    assert!(stdout.contains("9.99"), "Should contain price");
}

#[test]
fn test_peek_single_row() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "col1,col2,col3").unwrap();
    writeln!(file, "A,B,C").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("col1"), "Should contain header");
    assert!(stdout.contains("A"), "Should contain data");
    assert!(stdout.contains("B"), "Should contain data");
    assert!(stdout.contains("C"), "Should contain data");
}

#[test]
fn test_peek_empty_values() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c").unwrap();
    writeln!(file, "1,,3").unwrap();
    writeln!(file, ",2,").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have headers and data
    assert!(stdout.contains("a"), "Should contain header a");
    assert!(stdout.contains("b"), "Should contain header b");
    assert!(stdout.contains("c"), "Should contain header c");
}

#[test]
fn test_peek_long_values() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "short,very_long_column_name").unwrap();
    writeln!(file, "A,This is a very long value").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should adjust column widths
    assert!(
        stdout.contains("very_long_column_name"),
        "Should contain long header"
    );
    assert!(
        stdout.contains("This is a very long value"),
        "Should contain long value"
    );
}

#[test]
fn test_peek_piped_input() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "x,y").unwrap();
    writeln!(file, "1,2").unwrap();

    let file_content = std::fs::read_to_string(file.path()).unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").write_stdin(file_content).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("x"), "Should handle piped input");
    assert!(stdout.contains("y"), "Should handle piped input");
}

#[test]
fn test_peek_many_columns() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c,d,e,f,g,h").unwrap();
    writeln!(file, "1,2,3,4,5,6,7,8").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should display all columns
    for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
        assert!(stdout.contains(letter), "Should contain column {}", letter);
    }
}

#[test]
fn test_peek_header_separator() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "col1,col2").unwrap();
    writeln!(file, "A,B").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have headers and data
    assert!(stdout.contains("col1"), "Should contain col1");
    assert!(stdout.contains("col2"), "Should contain col2");
    assert!(stdout.contains("A"), "Should contain A");
    assert!(stdout.contains("B"), "Should contain B");
}

#[test]
fn test_peek_with_quoted_fields() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "name,description").unwrap();
    writeln!(file, "Alice,\"Software Engineer\"").unwrap();
    writeln!(file, "Bob,\"Product Manager\"").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Alice"), "Should contain Alice");
    assert!(stdout.contains("Bob"), "Should contain Bob");
    assert!(
        stdout.contains("Software Engineer") || stdout.contains("Product Manager"),
        "Should contain job titles"
    );
}

#[test]
fn test_peek_max_rows_zero() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id,value").unwrap();
    writeln!(file, "1,A").unwrap();
    writeln!(file, "2,B").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek")
        .arg("--number-rows")
        .arg("0")
        .arg(file.path())
        .assert()
        .success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show header but no data rows
    assert!(stdout.contains("id"), "Should show header");
    assert!(stdout.contains("value"), "Should show header");
    assert!(
        !stdout.contains("1") || !stdout.contains("A"),
        "Should not show data"
    );
}

#[test]
fn test_peek_single_column() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "value").unwrap();
    writeln!(file, "A").unwrap();
    writeln!(file, "B").unwrap();

    let mut cmd = cargo_bin_cmd!("clw");
    cmd.arg("peek").arg(file.path()).assert().success();

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("value"), "Should contain header");
    assert!(stdout.contains("A"), "Should contain first value");
    assert!(stdout.contains("B"), "Should contain second value");
}
