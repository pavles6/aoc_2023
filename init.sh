#!/bin/bash

# Base directory of the Rust project, change as needed
base_directory=$(pwd)

# Loop to create subdirectories and files
for i in {3..25}
do
    # Create a subdirectory for each day
    day_directory="$base_directory/src/day$i"
    mkdir -p "$day_directory"

    # Create a Rust file in the subdirectory
    filename="$day_directory/mod.rs"

    echo "use std::error::Error;" >> "$filename"
    echo "" >> "$filename"
    echo "// Module for day $i" >> "$filename"
    echo "" >> "$filename"
    echo "pub fn run() -> Result<(), Box<dyn Error>> {" >> "$filename"
    echo "    println!(\"Day $i module running...\");" >> "$filename"
    echo "" >> "$filename"
    echo "    Ok(())" >> "$filename"
    echo "}" >> "$filename"
done

# Create a lib.rs file
lib_file="$base_directory/src/lib.rs"
echo "// lib.rs" > "$lib_file"
for i in {1..25}
do
    echo "pub mod day$i;" >> "$lib_file"
done

echo "25 Rust module directories and files created, along with lib.rs."