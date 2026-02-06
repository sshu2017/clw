use crate::utils::{csv_writer, detect_delimiter, input_reader};
use csv::ReaderBuilder;
use std::collections::HashSet;
use std::error::Error;
use std::io;

pub fn filter_rows(
    path: Option<&str>,
    column: &str,
    values: &str,
    include_header: bool,
) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);

    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv.headers()?.clone();

    // Find the column index
    let col_idx = headers.iter().position(|h| h == column).ok_or_else(|| {
        let available: Vec<&str> = headers.iter().collect();
        format!(
            "\nColumn '{}' not found in CSV.\nAvailable columns: {}",
            column,
            available.join(", ")
        )
    })?;

    // Parse the filter values into a HashSet for efficient lookup
    let filter_values: HashSet<String> = values.split(',').map(|v| v.trim().to_string()).collect();

    // Create CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter);

    // Print header if requested
    if include_header {
        writer.write_record(&headers.iter().collect::<Vec<_>>())?;
    }

    // Filter and print matching rows
    let mut match_count = 0;
    for result in csv.records() {
        let record = result?;

        // Check if the value in the specified column matches any filter value
        if let Some(cell_value) = record.get(col_idx) {
            if filter_values.contains(cell_value) {
                writer.write_record(&record.iter().collect::<Vec<_>>())?;
                match_count += 1;
            }
        }
    }

    // Inform user if no matches were found
    if match_count == 0 {
        eprintln!("\nWarning: No rows matched the filter criteria.");
        eprintln!(
            "Column: '{}', Values: {}",
            column,
            filter_values
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    Ok(())
}
