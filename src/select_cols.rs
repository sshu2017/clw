use crate::utils::{csv_writer, detect_delimiter, input_reader};
use colored::Colorize;
use csv::ReaderBuilder;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

pub fn select_cols(path: Option<&str>, columns: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);

    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv.headers()?;

    // Parse the requested column names
    let requested_cols: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();

    // Check for duplicates
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();
    for col_name in &requested_cols {
        if !seen.insert(col_name) && !duplicates.contains(col_name) {
            duplicates.push(col_name);
        }
    }

    // If duplicates found, ask for confirmation
    if !duplicates.is_empty() {
        eprintln!(
            "{}",
            "⚠ Warning: Duplicate columns detected!".yellow().bold()
        );
        eprintln!(
            "  Duplicated columns: {}",
            duplicates
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ")
        );
        eprint!("  Do you want to proceed? [y/N]: ");
        io::stderr().flush()?;

        // Read from /dev/tty to get user input from terminal, not from stdin
        // This ensures we don't conflict with piped CSV data
        use atty::{is, Stream};
        use std::fs::File;
        use std::io::BufRead;

        let response = if let Ok(tty) = File::open("/dev/tty") {
            let mut tty_reader = io::BufReader::new(tty);
            let mut response = String::new();
            tty_reader.read_line(&mut response)?;
            response.trim().to_lowercase()
        } else if is(Stream::Stdin) {
            // Stdin is a TTY (terminal), safe to read from it
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            response.trim().to_lowercase()
        } else {
            // In non-interactive mode (tests, pipes), default to "no"
            eprintln!("  (Non-interactive mode: defaulting to 'no')");
            "n".to_string()
        };

        if response != "y" && response != "yes" {
            eprintln!("Operation cancelled.");
            return Ok(());
        }
    }

    // Find the indices of the requested columns
    let mut col_indices: Vec<usize> = Vec::new();
    for col_name in &requested_cols {
        match headers.iter().position(|h| h == *col_name) {
            Some(idx) => col_indices.push(idx),
            None => {
                return Err(format!(
                    "Column '{}' not found in CSV. Available columns: {}",
                    col_name,
                    headers.iter().collect::<Vec<_>>().join(", ")
                )
                .into());
            }
        }
    }

    // Use CSV writer for proper quoting
    let stdout = io::stdout();
    let mut writer = csv_writer(stdout.lock(), delimiter);

    // Print selected headers
    writer.write_record(&requested_cols)?;

    // Print selected columns for each row
    for result in csv.records() {
        let record = result?;
        let selected_values: Vec<&str> = col_indices
            .iter()
            .map(|&idx| record.get(idx).unwrap_or(""))
            .collect();
        writer.write_record(&selected_values)?;
    }

    writer.flush()?;
    Ok(())
}
