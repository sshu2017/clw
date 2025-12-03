#!/bin/bash

# Benchmark CLW against other CSV tools
# Run from repo root: bash benchmarks/compare_tools.sh

CLW="./target/release/clw"

echo "=========================================="
echo "CLW vs Other CSV Tools Benchmarking"
echo "=========================================="
echo ""

# Check that clw is built
if [ ! -f "$CLW" ]; then
    echo "Error: clw not found at $CLW"
    echo "Run: cargo build --release"
    exit 1
fi

# Check that benchmark data exists
if [ ! -f "benchmarks/data_100k.csv" ]; then
    echo "Error: Benchmark data not found"
    echo "Run: bash benchmarks/generate_data.sh"
    exit 1
fi

# Check for available tools
echo "Checking available CSV tools..."
if which csvtool >/dev/null 2>&1; then HAS_CSVTOOL="yes"; else HAS_CSVTOOL="no"; fi
if which csvstat >/dev/null 2>&1; then HAS_CSVKIT="yes"; else HAS_CSVKIT="no"; fi
if which qsv >/dev/null 2>&1; then HAS_QSV="yes"; else HAS_QSV="no"; fi
if which xan >/dev/null 2>&1; then HAS_XAN="yes"; else HAS_XAN="no"; fi

echo "  csvtool: $HAS_CSVTOOL"
echo "  csvkit:  $HAS_CSVKIT"
echo "  qsv:     $HAS_QSV"
echo "  xan:     $HAS_XAN"
echo ""

# 1. SELECT/COL - Extract columns
if [ "$HAS_CSVTOOL" = "yes" ]; then
    echo "1. SELECT: Extract columns (name, age)"
    echo "----------------------------------------"
    hyperfine --warmup 2 --shell=none \
        "$CLW select -c name,age benchmarks/data_100k.csv" \
        "/usr/bin/csvtool namedcol name,age benchmarks/data_100k.csv"
    echo ""
fi

# 2. HEAD/PEEK - Show first N rows
if [ "$HAS_CSVTOOL" = "yes" ]; then
    echo "2. PEEK/HEAD: Show first 10 rows"
    echo "----------------------------------------"
    hyperfine --warmup 2 --shell=none \
        "$CLW peek -n 10 benchmarks/data_100k.csv" \
        "/usr/bin/csvtool head 11 benchmarks/data_100k.csv"
    echo ""
fi

# 3. TRANSPOSE
if [ "$HAS_CSVTOOL" = "yes" ]; then
    echo "3. TRANSPOSE: Transpose rows and columns (10K dataset)"
    echo "----------------------------------------"
    hyperfine --warmup 2 --shell=none \
        "$CLW transpose benchmarks/data_10k.csv" \
        "/usr/bin/csvtool transpose benchmarks/data_10k.csv"
    echo ""
fi

# 4. STATS - with csvkit (if available)
if [ "$HAS_CSVKIT" = "yes" ]; then
    echo "4. STATS: Column statistics"
    echo "----------------------------------------"
    hyperfine --warmup 2 \
        "$CLW stats -c score benchmarks/data_100k.csv > /dev/null" \
        "csvstat -c score benchmarks/data_100k.csv > /dev/null"
    echo ""
fi

# 5. FREQ - with qsv (if available)
if [ "$HAS_QSV" = "yes" ]; then
    echo "5. FREQ: Frequency count"
    echo "----------------------------------------"
    hyperfine --warmup 2 \
        "$CLW freq -c city benchmarks/data_100k.csv > /dev/null" \
        "qsv frequency -s city benchmarks/data_100k.csv > /dev/null"
    echo ""
fi

# 6. SELECT - with qsv (if available)
if [ "$HAS_QSV" = "yes" ]; then
    echo "6. SELECT: Column selection (qsv comparison)"
    echo "----------------------------------------"
    hyperfine --warmup 2 \
        "$CLW select -c name,age benchmarks/data_100k.csv > /dev/null" \
        "qsv select name,age benchmarks/data_100k.csv > /dev/null"
    echo ""
fi

# 7. INFO - with qsv (if available)
if [ "$HAS_QSV" = "yes" ]; then
    echo "7. INFO: Dataset information"
    echo "----------------------------------------"
    hyperfine --warmup 2 \
        "$CLW info benchmarks/data_100k.csv > /dev/null" \
        "qsv stats benchmarks/data_100k.csv > /dev/null"
    echo ""
fi

echo "=========================================="
echo "Benchmarking Complete!"
echo ""
echo "To install more tools for comparison:"
echo "  csvkit (Python): pip install csvkit"
echo "  qsv (Rust):      cargo install qsv"
echo "  xan (Rust):      cargo install xan"
echo "=========================================="
