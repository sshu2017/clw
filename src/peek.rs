use crate::utils::{detect_delimiter, input_reader};
use colored::*;
use csv::ReaderBuilder;
use std::error::Error;

// Rainbow colors for columns (cycling through these)
const COLORS: &[&str] = &[
    "cyan",
    "green",
    "yellow",
    "magenta",
    "blue",
    "red",
    "bright_cyan",
    "bright_green",
];

pub fn peek(path: Option<&str>, number_rows: Option<usize>) -> Result<(), Box<dyn Error>> {
    // Default to 10 rows if not specified
    let rows_to_show = number_rows.unwrap_or(10);

    // Read input
    let mut reader = input_reader(path);
    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv_reader.headers()?.clone();

    // Only read the first N rows (performance improvement - don't read all rows!)
    let mut records: Vec<Vec<String>> = Vec::new();

    for result in csv_reader.records() {
        // Stop reading if we already have enough rows
        if records.len() >= rows_to_show {
            break;
        }

        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        records.push(row);
    }

    // Calculate column widths based on displayed rows only
    let num_cols = headers.len();
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for record in &records {
        for (i, value) in record.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(value.len());
            }
        }
    }

    // Print headers with colors
    for (i, header) in headers.iter().enumerate() {
        let color = COLORS[i % COLORS.len()];
        let colored_header = apply_color(header, color).bold();
        print!("{:<width$}", colored_header, width = col_widths[i]);
        if i < headers.len() - 1 {
            print!("  "); // Two spaces between columns
        }
    }
    println!();

    // Print data rows
    for record in &records {
        for (i, value) in record.iter().enumerate() {
            if i < num_cols {
                let color = COLORS[i % COLORS.len()];
                let colored_value = apply_color(value, color);
                print!("{:<width$}", colored_value, width = col_widths[i]);
                if i < num_cols - 1 {
                    print!("  "); // Two spaces between columns
                }
            }
        }
        println!();
    }

    Ok(())
}

fn apply_color(text: &str, color: &str) -> ColoredString {
    match color {
        "cyan" => text.cyan(),
        "green" => text.green(),
        "yellow" => text.yellow(),
        "magenta" => text.magenta(),
        "blue" => text.blue(),
        "red" => text.red(),
        "bright_cyan" => text.bright_cyan(),
        "bright_green" => text.bright_green(),
        _ => text.white(),
    }
}
