# CLW - CSV Light Wizard

> **Pronounced "CLAW"** - because every Rust tool deserves a crab-themed name.

[![PyPI version](https://badge.fury.io/py/clw.svg)](https://pypi.org/project/clw/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

CLW is a blazing-fast CSV manipulation tool written in Rust, designed to make working with CSV files from the command line intuitive and efficient. This Python package provides a convenient installer that downloads the pre-built binary for your platform.

## Performance

**10-400x faster than csvkit** (Python), the most popular CSV tool among data scientists:

- **Column selection**: 14.8x faster
- **Statistics**: 122x faster
- **Frequency analysis**: 6x faster
- **Dataset overview**: 404x faster

## Features

- **Lightning Fast**: Built with Rust for maximum performance - orders of magnitude faster than Python tools
- **Auto-detection**: Automatically detects delimiters (comma, pipe, tab, etc.)
- **Data Analysis**: Built-in statistics and frequency analysis with visualization
- **Data Transformation**: Transpose, stack, and paste operations
- **Smart Filtering**: Filter by values or patterns with regex support
- **Random Sampling**: Sample rows or percentages with reproducible seeds
- **Flexible I/O**: Works with files or stdin/stdout for easy piping
- **Lightweight**: 1.7MB single binary, no dependencies

## Installation

Install via pip:

```bash
pip install clw
```

The package will automatically download the appropriate pre-built binary for your platform during installation.

### Supported Platforms

- Linux x86_64 (musl)
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64

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

## Usage

For detailed usage instructions, see the [main repository](https://github.com/sshu2017/clw).

## Building from Source

If you prefer to build from source or your platform is not supported:

```bash
git clone https://github.com/sshu2017/clw.git
cd clw/
cargo build --release
sudo cp target/release/clw /usr/local/bin/
```

## Why CLW?

Born from 10+ years of day-to-day data science work, CLW is a Swiss Army knife that brings together the essential CSV operations you actually need in your workflow. Unlike other tools, CLW focuses on:

- **Speed**: Rust performance for large datasets
- **Simplicity**: Intuitive commands for common tasks
- **Practicality**: Features that data scientists actually use daily
- **Reliability**: Single binary with no runtime dependencies

## Acknowledgments

CLW was inspired by [csvkit](https://csvkit.readthedocs.io/), [xsv](https://github.com/BurntSushi/xsv), and other excellent CSV tools.

## License

MIT License - see the [LICENSE](https://github.com/sshu2017/clw/blob/main/LICENSE) file for details.

## Links

- [GitHub Repository](https://github.com/sshu2017/clw)
- [Issue Tracker](https://github.com/sshu2017/clw/issues)
- [Documentation](https://github.com/sshu2017/clw)