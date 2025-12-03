use crate::utils::{detect_delimiter, input_reader};
use colored::*;
use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::error::Error;

pub fn column_stats(path: Option<&str>, column: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = input_reader(path);
    let delimiter = detect_delimiter(&mut *reader)?;

    let mut csv = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter as u8)
        .from_reader(reader);

    let headers = csv.headers()?.clone();

    // Find column index
    let col_idx = headers.iter().position(|h| h == column).ok_or_else(|| {
        let available: Vec<&str> = headers.iter().collect();
        format!(
            "Column '{}' not found in CSV.\nAvailable columns: {}",
            column,
            available.join(", ")
        )
    })?;

    // Collect all values with progress indicator
    let mut values: Vec<String> = Vec::new();

    // Create progress spinner (only shows if stderr is a TTY)
    let spinner = if atty::is(atty::Stream::Stderr) {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.set_message("Reading data...");
        Some(pb)
    } else {
        None
    };

    for result in csv.records() {
        let record = result?;
        if let Some(val) = record.get(col_idx) {
            values.push(val.to_string());
        }

        // Update spinner every 1000 rows
        if let Some(ref pb) = spinner {
            if values.len() % 1000 == 0 {
                pb.set_message(format!("Reading data... {} rows", values.len()));
                pb.tick();
            }
        }
    }

    // Finish spinner
    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    // Try to parse as numeric
    let mut numeric_values: Vec<f64> = Vec::new();
    let mut null_count = 0;
    let mut invalid_count = 0;

    for val in &values {
        if val.trim().is_empty() {
            null_count += 1;
        } else if let Ok(num) = val.trim().parse::<f64>() {
            numeric_values.push(num);
        } else {
            invalid_count += 1;
        }
    }

    let total_count = values.len();
    let valid_numeric = numeric_values.len();

    // Decide if this is a numeric column (more than 50% of non-null values are numeric)
    let non_null_count = total_count - null_count;
    let is_numeric = non_null_count > 0 && (valid_numeric as f64 / non_null_count as f64) > 0.5;

    if is_numeric {
        print_numeric_stats(
            column,
            total_count,
            null_count,
            invalid_count,
            &mut numeric_values,
        );
    } else {
        print_categorical_stats(column, total_count, null_count, &values);
    }

    Ok(())
}

fn print_numeric_stats(
    column: &str,
    total: usize,
    null_count: usize,
    invalid_count: usize,
    values: &mut Vec<f64>,
) {
    println!(
        "\n{}",
        format!("Column '{}' Statistics (Numeric)", column)
            .green()
            .bold()
    );
    println!();
    println!("{:<13} {}", "Count:".green(), total);
    println!("{:<13} {}", "Null/Empty:".green(), null_count);
    if invalid_count > 0 {
        println!(
            "{:<13} {} (could not convert to number)",
            "Invalid:".green(),
            invalid_count
        );
    }

    if values.is_empty() {
        println!("\nNo numeric values to analyze.");
        return;
    }

    // Calculate statistics
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    // Standard deviation
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();

    // Sort for min, max, and percentiles
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = values[0];
    let max = values[values.len() - 1];

    println!();
    println!("{:<13} {:.2}", "Mean:".green(), mean);
    println!("{:<13} {:.2}", "Std Dev:".green(), std_dev);
    println!("{:<13} {:.2}", "Min:".green(), min);
    println!("{:<13} {:.2}", "Max:".green(), max);

    // Percentiles
    println!();
    println!("{}:", "Percentiles".green());
    println!(
        "  {}{:.2}",
        format!("{:<12}", "1%:").green(),
        percentile(values, 1.0)
    );
    println!(
        "  {}{:.2}",
        format!("{:<12}", "25%:").green(),
        percentile(values, 25.0)
    );
    println!(
        "  {}{:.2}",
        format!("{:<12}", "50%:").green(),
        percentile(values, 50.0)
    );
    println!(
        "  {}{:.2}",
        format!("{:<12}", "75%:").green(),
        percentile(values, 75.0)
    );
    println!(
        "  {}{:.2}",
        format!("{:<12}", "99%:").green(),
        percentile(values, 99.0)
    );
    println!();
}

fn print_categorical_stats(column: &str, total: usize, null_count: usize, values: &[String]) {
    println!(
        "\n{}",
        format!("Column '{}' Statistics (Categorical)", column)
            .green()
            .bold()
    );
    println!();
    println!("{:<13} {}", "Total Count:".green(), total);
    println!("{:<13} {}", "Null/Empty:".green(), null_count);

    // Count frequencies (excluding empty values)
    let mut freq_map: HashMap<String, usize> = HashMap::new();
    for val in values {
        if !val.trim().is_empty() {
            *freq_map.entry(val.clone()).or_insert(0) += 1;
        }
    }

    let unique_count = freq_map.len();
    println!("{:<13} {}", "Unique:".green(), unique_count);

    if unique_count == 0 {
        println!("\nNo non-empty values to analyze.");
        return;
    }

    // Get top 3
    let mut freq_vec: Vec<(&String, &usize)> = freq_map.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!();
    println!("{}:", "Top 3 Most Frequent".green());
    let non_null_total = (total - null_count) as f64;
    for (i, (val, count)) in freq_vec.iter().take(3).enumerate() {
        let percentage = (**count as f64 / non_null_total) * 100.0;
        println!(
            "  {}. {:<20} {:>6}  ({:.1}%)",
            i + 1,
            val,
            count,
            percentage
        );
    }
    println!();
}

fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    let n = sorted_values.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return sorted_values[0];
    }

    // Linear interpolation between closest ranks
    let rank = (p / 100.0) * (n - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let fraction = rank - lower as f64;

    sorted_values[lower] + fraction * (sorted_values[upper] - sorted_values[lower])
}
