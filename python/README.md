# CLW - CSV Light Wizard

> **Pronounced "CLAW"** - because every Rust tool deserves a crab-themed name.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)]()

CLW is a blazing-fast CSV manipulation tool written in Rust, designed to make working with CSV files from the command line intuitive and efficient. Born from 10+ years of day-to-day data science work, CLW is a Swiss Army knife that brings together the essential CSV operations you actually need in your workflow.

## Features

- **Lightning Fast**: Built with Rust for maximum performance - See [BENCHMARKS.md](BENCHMARKS.md) for details.
- **Auto-detection**: Automatically detects delimiters (comma, pipe, tab, and space.)
- **Data Analysis**: Built-in statistics and frequency analysis
- **Data Transformation**: Transpose, stack, and paste operations
- **Smart Filtering**: Filter by values 
- **Random Sampling**: Sample rows with reproducible seeds
- **Flexible I/O**: Works with files or stdin/stdout for easy piping
- **Lightweight**: 1.7MB single binary

## Installation

### Pip install (recommended)
```bash
pip install clw
```

### From Source

```bash
git clone https://github.com/sshu2017/clw.git
cd clw/
cargo build --release
sudo cp target/release/clw /usr/local/bin/
```

## Quick Start

```bash
# Pretty-print a CSV with rainbow colors
clw peek data.csv

# Get basic statistics for a column
clw stats --column price products.csv

# Show frequency distribution with a plot
clw freq --column category --plot inventory.csv

# Transpose rows and columns
clw transpose data.csv > transposed.csv

# Stack two CSV files vertically
clw stack file1.csv file2.csv > combined.csv
```

> See the **[USAGE.md](./USAGE.md)** for detailed usage.

##  Acknowledgments

CLW was inspired by:
- [csvkit](https://csvkit.readthedocs.io/) - a comprehensive CSV toolkit in Python
- [csvtool](https://github.com/Chris00/ocaml-csv) - a fast CSV processing tool in OCaml
- [xsv](https://github.com/BurntSushi/xsv) - a CSV toolkit in Rust (now unmaintained)
- [xan](https://github.com/medialab/xan) and [qsv](https://github.com/dathere/qsv) - two feature-rich CSV toolkits in Rust (though larger in size)

Special thanks to the Rust community for excellent libraries:
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [csv](https://github.com/BurntSushi/rust-csv) - CSV reading/writing
- [colored](https://github.com/mackwic/colored) - Terminal colors

## Star History

If you find CLW useful, please consider giving it a star on GitHub.
