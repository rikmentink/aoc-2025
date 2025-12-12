#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: ./run.sh <day_number>"
    exit 1
fi

# Pad the day number with a leading zero if needed
DAY=$(printf "%02d" $1)

# Check if the binary source exists
if [ ! -f "src/bin/day$DAY.rs" ]; then
    echo "Error: Source file src/bin/day$DAY.rs does not exist."
    exit 1
fi

cargo run --bin "day$DAY"

