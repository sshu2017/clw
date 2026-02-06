use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use std::error::Error;
use std::io;

pub fn paste(file1_path: &str, file2_path: &str) -> Result<(), Box<dyn Error>> {
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

    // Use CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter1);

    // Combine and write headers
    let combined_header: Vec<&str> = headers1.iter().chain(headers2.iter()).collect();
    writer.write_record(&combined_header)?;

    // Stream records from both files simultaneously
    let mut iter1 = csv1.records();
    let mut iter2 = csv2.records();
    let mut row_num = 0;

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(Ok(record1)), Some(Ok(record2))) => {
                // Combine and write row
                let combined_row: Vec<&str> = record1.iter().chain(record2.iter()).collect();
                writer.write_record(&combined_row)?;
                row_num += 1;
            }
            (None, None) => break, // Both files ended at same time - good!
            (Some(_), None) => {
                return Err(
                    format!("File 1 has more rows than File 2 (at row {})", row_num + 1).into(),
                );
            }
            (None, Some(_)) => {
                return Err(
                    format!("File 2 has more rows than File 1 (at row {})", row_num + 1).into(),
                );
            }
            (Some(Err(e)), _) | (_, Some(Err(e))) => return Err(e.into()),
        }
    }

    writer.flush()?;
    Ok(())
}
