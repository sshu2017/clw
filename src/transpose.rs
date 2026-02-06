use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use std::error::Error;
use std::io;

pub fn transpose(path: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Read input
    let mut reader = input_reader(path);
    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false) // Treat all rows the same for transpose
        .flexible(true) // Allow rows with different column counts
        .delimiter(delimiter as u8)
        .from_reader(reader);

    // Read all records into a vector of vectors
    let mut rows: Vec<Vec<String>> = Vec::new();

    for result in csv_reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        rows.push(row);
    }

    if rows.is_empty() {
        return Ok(());
    }

    // Find the maximum number of columns
    let max_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Create CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter);

    // Transpose: rows become columns, columns become rows
    for col_idx in 0..max_cols {
        let transposed_row: Vec<String> = rows
            .iter()
            .map(|row| {
                row.get(col_idx)
                    .map(|s| s.to_string())
                    .unwrap_or_else(String::new)
            })
            .collect();

        writer.write_record(&transposed_row)?;
    }

    Ok(())
}
