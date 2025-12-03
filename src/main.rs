use clap::{Parser, Subcommand};
use std::error::Error;

mod filter;
mod freq;
mod info;
mod paste;
mod peek;
mod sample_rows;
mod select_cols;
mod show_header;
mod stack;
mod stats;
mod transpose;
mod utils;
use filter::filter_rows;
use freq::freq;
use info::get_info;
use paste::paste;
use peek::peek;
use sample_rows::sample_rows;
use select_cols::select_cols;
use show_header::show_header;
use stack::stack;
use stats::column_stats;
use transpose::transpose;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the headers with line numbers
    Header { file: Option<String> },
    /// Shows dataset info: number of rows, columns, and warns about inconsistent rows
    Info { file: Option<String> },
    /// Selects/cuts columns by names
    Select {
        /// Comma-separated list of column names to select (e.g., "name,age,city")
        #[arg(short, long)]
        columns: String,

        file: Option<String>,
    },
    /// Randomly samples rows from CSV
    Sample {
        /// Number of rows to sample
        #[arg(short, long)]
        rows: usize,

        /// Random seed for reproducible sampling (default: 67)
        #[arg(long, default_value = "67")]
        seed: u64,

        /// Exclude header row from output
        #[arg(long)]
        no_header: bool,

        file: Option<String>,
    },
    /// Filters rows based on column values
    Filter {
        /// Column name to filter on
        #[arg(short, long)]
        column: String,

        /// Comma-separated list of values to match (e.g., "100,200,300")
        #[arg(short, long)]
        value: String,

        /// Include header row in output
        #[arg(long)]
        keep_header: bool,

        file: Option<String>,
    },
    /// Shows statistics for a column
    Stats {
        /// Column name to analyze
        #[arg(short, long)]
        column: String,

        file: Option<String>,
    },
    /// Shows frequency counts for unique values in a column
    Freq {
        /// Column name to analyze
        #[arg(short, long)]
        column: String,

        /// Display horizontal bar plot
        #[arg(short, long)]
        plot: bool,

        /// Sort by index (value) - alphabetically or numerically (default: sort by frequency)
        #[arg(long)]
        sort_index: bool,

        file: Option<String>,
    },
    /// Stack two CSV files vertically (keeping one header)
    Stack { file1: String, file2: String },
    /// Paste two CSV files horizontally (side by side)
    Paste { file1: String, file2: String },
    /// Transpose rows and columns
    Transpose { file: Option<String> },
    /// Pretty-print CSV header and first N rows with rainbow colors
    Peek {
        /// Number of rows to display (default: 10)
        #[arg(short, long)]
        number_rows: Option<usize>,

        file: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Header { file } => show_header(file.as_deref())?,
        Commands::Info { file } => get_info(file.as_deref())?,
        Commands::Select { columns, file } => select_cols(file.as_deref(), &columns)?,
        Commands::Sample {
            rows,
            seed,
            no_header,
            file,
        } => sample_rows(file.as_deref(), rows, seed, !no_header)?,
        Commands::Filter {
            column,
            value,
            keep_header,
            file,
        } => filter_rows(file.as_deref(), &column, &value, keep_header)?,
        Commands::Stats { column, file } => column_stats(file.as_deref(), &column)?,
        Commands::Freq {
            column,
            plot,
            sort_index,
            file,
        } => freq(file.as_deref(), &column, plot, sort_index)?,
        Commands::Stack { file1, file2 } => stack(&file1, &file2)?,
        Commands::Paste { file1, file2 } => paste(&file1, &file2)?,
        Commands::Transpose { file } => transpose(file.as_deref())?,
        Commands::Peek { number_rows, file } => peek(file.as_deref(), number_rows)?,
    }

    Ok(())
}
