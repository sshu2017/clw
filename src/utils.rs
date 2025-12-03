use atty::{is, Stream};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn input_reader(path: Option<&str>) -> Box<dyn BufRead> {
    match path {
        Some(p) => Box::new(BufReader::new(File::open(p).expect("Cannot open file"))),
        None => {
            if is(Stream::Stdin) {
                panic!("Expected a file or piped input, but stdin is a TTY");
            }
            Box::new(BufReader::new(io::stdin()))
        }
    }
}

pub fn detect_delimiter(reader: &mut dyn BufRead) -> Result<char, Box<dyn Error>> {
    // Peek at buffer without consuming it
    let buffer = reader.fill_buf()?;

    if buffer.is_empty() {
        return Err("File is empty or contains no data".into());
    }

    // Find first newline position
    let first_line_end = buffer
        .iter()
        .position(|&b| b == b'\n')
        .unwrap_or(buffer.len());

    // Convert bytes to string (still just viewing, not consuming)
    let first_line =
        std::str::from_utf8(&buffer[..first_line_end]).map_err(|_| "Invalid UTF-8 in file")?;

    if first_line.trim().is_empty() {
        return Err("File is empty or contains no data".into());
    }

    // Detect delimiter - data is still in the reader!
    let delimiter = if first_line.contains('|') { '|' } else { ',' };
    Ok(delimiter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_detect_delimiter_comma() {
        let data = "name,age,city\nAlice,30,NYC\n";
        let mut reader = Cursor::new(data);
        let delimiter = detect_delimiter(&mut reader).unwrap();
        assert_eq!(delimiter, ',');
    }

    #[test]
    fn test_detect_delimiter_pipe() {
        let data = "name|age|city\nBob|25|LA\n";
        let mut reader = Cursor::new(data);
        let delimiter = detect_delimiter(&mut reader).unwrap();
        assert_eq!(delimiter, '|');
    }

    #[test]
    fn test_detect_delimiter_defaults_to_comma() {
        let data = "name age city\nCharlie 35 Boston\n";
        let mut reader = Cursor::new(data);
        let delimiter = detect_delimiter(&mut reader).unwrap();
        assert_eq!(delimiter, ',', "Should default to comma when no pipe found");
    }

    #[test]
    fn test_detect_delimiter_empty_file() {
        let data = "";
        let mut reader = Cursor::new(data);
        let result = detect_delimiter(&mut reader);
        assert!(result.is_err(), "Should return error for empty file");
        assert_eq!(
            result.unwrap_err().to_string(),
            "File is empty or contains no data"
        );
    }

    #[test]
    fn test_detect_delimiter_only_whitespace() {
        let data = "   \n  \n";
        let mut reader = Cursor::new(data);
        let result = detect_delimiter(&mut reader);
        assert!(
            result.is_err(),
            "Should return error for whitespace-only file"
        );
    }
}
