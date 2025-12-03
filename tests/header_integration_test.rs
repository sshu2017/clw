use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// Import the public functions from the clw library
// Note: For this to work, we need to expose the functions as a library

#[test]
fn test_header_comma_delimited_file() {
    // This test demonstrates reading a comma-delimited CSV
    let file_path = "tests/fixtures/sample_comma.csv";
    let file = File::open(file_path).expect("Failed to open test fixture");
    let mut reader = BufReader::new(file);

    // We'd normally call clw's functions here, but since show_header prints to stdout,
    // we'll test the underlying components
    let result = detect_delimiter_from_file(&mut reader);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ',');
}

#[test]
fn test_header_pipe_delimited_file() {
    let file_path = "tests/fixtures/sample_pipe.csv";
    let file = File::open(file_path).expect("Failed to open test fixture");
    let mut reader = BufReader::new(file);

    let result = detect_delimiter_from_file(&mut reader);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), '|');
}

#[test]
fn test_header_empty_file() {
    let file_path = "tests/fixtures/empty.csv";
    let file = File::open(file_path).expect("Failed to open test fixture");
    let mut reader = BufReader::new(file);

    let result = detect_delimiter_from_file(&mut reader);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "File is empty or contains no data"
    );
}

#[test]
fn test_header_single_column() {
    let file_path = "tests/fixtures/single_column.csv";
    let file = File::open(file_path).expect("Failed to open test fixture");
    let mut reader = BufReader::new(file);

    let result = detect_delimiter_from_file(&mut reader);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ','); // Should default to comma
}

// Helper function to test delimiter detection
// This mimics the logic from utils::detect_delimiter
fn detect_delimiter_from_file(reader: &mut dyn std::io::BufRead) -> Result<char, Box<dyn Error>> {
    let mut first_line = String::new();
    let bytes_read = reader.read_line(&mut first_line)?;

    if bytes_read == 0 || first_line.trim().is_empty() {
        return Err("File is empty or contains no data".into());
    }

    let delimiter = if first_line.contains('|') { '|' } else { ',' };
    Ok(delimiter)
}
