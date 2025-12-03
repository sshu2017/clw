use crate::utils::{detect_delimiter, input_reader};
use colored::Colorize;
use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;

pub fn get_info(path: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);

    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .flexible(true) // Allow rows with different number of fields
        .from_reader(reader);

    let headers = csv.headers()?;
    let num_columns = headers.len();

    let mut num_rows = 0;
    let mut inconsistent_rows = Vec::new();

    // Create progress spinner (only shows if stderr is a TTY)
    let spinner = if atty::is(atty::Stream::Stderr) {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.set_message("Analyzing data...");
        Some(pb)
    } else {
        None
    };

    for (idx, result) in csv.records().enumerate() {
        let record = result?;
        num_rows += 1;

        // Check if this row has a different number of fields
        if record.len() != num_columns {
            inconsistent_rows.push((idx + 2, record.len())); // +2 because: 0-indexed + header row
        }

        // Update spinner every 1000 rows
        if let Some(ref pb) = spinner {
            if num_rows % 1000 == 0 {
                pb.set_message(format!("Analyzing data... {} rows", num_rows));
                pb.tick();
            }
        }
    }

    // Finish spinner
    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    // Print summary
    println!("\n{}", "Dataset Info:".green().bold());
    println!("  Rows:    {}", num_rows);
    println!("  Columns: {}", num_columns);

    // Warn about inconsistent rows
    if !inconsistent_rows.is_empty() {
        println!(
            "\n{}",
            "⚠ WARNING: Inconsistent row lengths detected!"
                .yellow()
                .bold()
        );
        println!("  Expected {} fields per row, but found:", num_columns);

        // Show first few inconsistent rows
        let display_limit = 5;
        for (row_num, field_count) in inconsistent_rows.iter().take(display_limit) {
            println!("    Row {}: {} fields", row_num, field_count);
        }

        if inconsistent_rows.len() > display_limit {
            println!(
                "    ... and {} more inconsistent rows",
                inconsistent_rows.len() - display_limit
            );
        }

        println!(
            "  Total inconsistent rows: {}/{}",
            inconsistent_rows.len(),
            num_rows
        );
    }

    Ok(())
}
