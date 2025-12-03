# CLW - CSV Light Wizard

> **Pronounced "CLAW"** - because every Rust tool deserves a crab-themed name.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)]()

CLW is a blazing-fast CSV manipulation tool written in Rust, designed to make working with CSV files from the command line intuitive and efficient. Born from 10+ years of day-to-day data science work, CLW is a Swiss Army knife that brings together the essential CSV operations you actually need in your workflow.

## Commands

### Basic Inspection

#### `header` - Display column headers with line numbers
```bash
clw header data.csv
cat data.csv | clw header
```

#### `info` - Show dataset information
```bash
clw info data.csv
# Output: number of rows, columns, warnings about inconsistent rows
```

#### `look` - Pretty-print with rainbow colors
```bash
clw look data.csv
clw look --max-rows 20 data.csv
cat data.csv | clw look
```
**Example output:**
```
Product   Category     Price   Stock  Rating
Laptop    Electronics  999.99  45     4.5
Mouse     Electronics  29.99   120    4.2
Keyboard  Electronics  79.99   85     4.7
```
*(Each column displayed in a different color)*

### Filtering & Selection

#### `select` - Select specific columns
```bash
clw select --columns name,age,city data.csv
clw select -c id,price,stock products.csv
```

#### `sample` - Random sampling
```bash
# Sample 100 rows
clw sample --rows 100 data.csv

# Sample 20% of rows
clw sample --frac 20 data.csv

# Sample without header
clw sample --rows 50 --no-header data.csv
```

#### `filter` - Filter rows by column values
```bash
# Filter by single value
clw filter --column status --value active users.csv

# Filter by multiple values
clw filter --column id --value 100,200,300 data.csv

# Output without header
clw filter -c category -v Electronics,Tools --no-header products.csv
```

### Analysis

#### `stats` - Column statistics
```bash
clw stats --column price products.csv
```
**Output:**
- Count
- Mean
- Standard Deviation
- Min/Max
- Quartiles (Q1, Median, Q3)

#### `freq` - Frequency analysis
```bash
# Basic frequency count
clw freq --column category products.csv

# With horizontal bar plot
clw freq --column status --plot tasks.csv

# Sort by value instead of frequency
clw freq --column name --sort-index users.csv
```
**Example output with `--plot`:**
```
Value        |                                                |  Count      Pct        CumPct
Electronics  |▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪|  120        60.00%      60.00%
Tools        |▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪                        |  50         25.00%      85.00%
Home         |▪▪▪▪▪▪▪▪▪▪▪▪                                    |  30         15.00%     100.00%
```

### Data Transformation

#### `transpose` - Transpose rows and columns
```bash
clw transpose data.csv
cat data.csv | clw transpose > transposed.csv
```
**Before:**
```
name,age,city
Alice,30,NYC
Bob,25,LA
```
**After:**
```
name,Alice,Bob
age,30,25
city,NYC,LA
```

### Merge & Stack

#### `stack` - Stack CSV files vertically
```bash
clw stack file1.csv file2.csv > combined.csv
```
- Keeps only one header
- Files must have identical headers and delimiters
- Preserves all data rows from both files

#### `paste` - Paste CSV files horizontally
```bash
clw paste file1.csv file2.csv > merged.csv
```
- Combines columns from both files side by side
- Files must have the same number of rows
- Files must have identical delimiters

## Usage Examples

### Pipeline Processing

CLW commands can be chained together using Unix pipes:

```bash
# Filter, sample, and pretty-print
cat large_file.csv | \
  clw filter -c status -v active | \
  clw sample --rows 100 | \
  clw look

# Analyze a specific subset
clw select -c name,price products.csv | \
  clw stats -c price

# Transpose and view
clw transpose matrix.csv | clw look --max-rows 10
```