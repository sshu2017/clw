#!/bin/bash

# Benchmark script comparing clw to standard Linux tools
# Run from repo root: bash benchmarks/run_benchmarks.sh

CLW="./target/release/clw"

echo "======================================"
echo "CLW vs Linux Tools Benchmarking"
echo "======================================"
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

echo "1. PEEK/HEAD: Show first 10 rows"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW peek -n 10 benchmarks/data_100k.csv > /dev/null" \
    "head -n 11 benchmarks/data_100k.csv > /dev/null"
echo ""

echo "2. PASTE: Combine two files horizontally"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW paste benchmarks/data_10k_part1.csv benchmarks/data_10k_part2.csv > /dev/null" \
    "paste -d, benchmarks/data_10k_part1.csv benchmarks/data_10k_part2.csv > /dev/null"
echo ""

echo "3. STACK: Combine files vertically"
echo "--------------------------------------"
# For fair comparison, Linux needs to skip one header
hyperfine --warmup 2 \
    "$CLW stack benchmarks/data_10k_part1.csv benchmarks/data_10k_part1.csv > /dev/null" \
    "cat benchmarks/data_10k_part1.csv <(tail -n +2 benchmarks/data_10k_part1.csv) > /dev/null"
echo ""

echo "4. SELECT: Extract specific columns"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW select -c name,age benchmarks/data_100k.csv > /dev/null" \
    "cut -d, -f2,3 benchmarks/data_100k.csv > /dev/null"
echo ""

echo "5. FILTER: Filter rows by condition"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW filter -c city -v City50 benchmarks/data_100k.csv > /dev/null" \
    "awk -F, '\$4 == \"City50\"' benchmarks/data_100k.csv > /dev/null"
echo ""

echo "6. FREQ: Count value frequencies"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW freq -c city benchmarks/data_100k.csv > /dev/null" \
    "tail -n +2 benchmarks/data_100k.csv | cut -d, -f4 | sort | uniq -c | sort -rn > /dev/null"
echo ""

echo "7. INFO: Get dataset information (clw only)"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW info benchmarks/data_100k.csv > /dev/null"
echo ""

echo "8. STATS: Column statistics (clw only)"
echo "--------------------------------------"
hyperfine --warmup 2 \
    "$CLW stats -c score benchmarks/data_100k.csv > /dev/null"
echo ""

echo "======================================"
echo "Benchmarking Complete!"
echo "======================================"
