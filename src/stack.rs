use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use std::error::Error;
use std::io;

pub fn stack(file1_path: &str, file2_path: &str) -> Result<(), Box<dyn Error>> {
    // Read first file
    let mut reader1 = input_reader(Some(file1_path));
    let delimiter1 = detect_delimiter(&mut *reader1)?;

    let mut csv1 = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter1 as u8)
        .from_reader(reader1);

    let headers1 = csv1.headers()?.clone();

    // Read second file
    let mut reader2 = input_reader(Some(file2_path));
    let delimiter2 = detect_delimiter(&mut *reader2)?;

    let mut csv2 = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter2 as u8)
        .from_reader(reader2);

    let headers2 = csv2.headers()?.clone();

    // Verify delimiters match
    if delimiter1 != delimiter2 {
        return Err(format!(
            "Files have different delimiters: '{}' vs '{}'",
            delimiter1, delimiter2
        )
        .into());
    }

    // Verify headers match
    if headers1.len() != headers2.len() {
        return Err(format!(
            "Files have different number of columns: {} vs {}",
            headers1.len(),
            headers2.len()
        )
        .into());
    }

    for (i, (h1, h2)) in headers1.iter().zip(headers2.iter()).enumerate() {
        if h1 != h2 {
            return Err(format!(
                "Headers don't match at position {}: '{}' vs '{}'",
                i, h1, h2
            )
            .into());
        }
    }

    // Use CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter1);

    // Print header once
    writer.write_record(headers1.iter().collect::<Vec<_>>())?;

    // Print all records from first file
    for result in csv1.records() {
        let record = result?;
        writer.write_record(record.iter().collect::<Vec<_>>())?;
    }

    // Print all records from second file
    for result in csv2.records() {
        let record = result?;
        writer.write_record(record.iter().collect::<Vec<_>>())?;
    }

    writer.flush()?;
    Ok(())
}
