#!/bin/bash

# Generate CSV files of different sizes for benchmarking
# Run from repo root: bash benchmarks/generate_data.sh

echo "Generating benchmark datasets..."
echo ""

# Small: 10K rows
echo "Generating 10K rows..."
{
    echo "id,name,age,city,score"
    for i in $(seq 1 10000); do
        echo "$i,User$i,$((20 + i % 50)),City$((i % 100)),$((i % 1000))"
    done
} > benchmarks/data_10k.csv

# Medium: 100K rows
echo "Generating 100K rows..."
{
    echo "id,name,age,city,score"
    for i in $(seq 1 100000); do
        echo "$i,User$i,$((20 + i % 50)),City$((i % 100)),$((i % 1000))"
    done
} > benchmarks/data_100k.csv

# Large: 1M rows
echo "Generating 1M rows..."
{
    echo "id,name,age,city,score"
    for i in $(seq 1 1000000); do
        echo "$i,User$i,$((20 + i % 50)),City$((i % 100)),$((i % 1000))"
    done
} > benchmarks/data_1m.csv

# Also create split files for paste/stack benchmarks
echo "Creating split files for paste benchmarks..."
head -n 5001 benchmarks/data_10k.csv | cut -d, -f1,2,3 > benchmarks/data_10k_part1.csv
head -n 5001 benchmarks/data_10k.csv | cut -d, -f4,5 > benchmarks/data_10k_part2.csv

echo ""
echo "Done! Generated files:"
ls -lh benchmarks/*.csv
