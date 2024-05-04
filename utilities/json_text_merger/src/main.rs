// Author: Mohammed H Alsaeygh
// Project: JSON Text Merger
//
// Description:
// This Rust command-line tool reads JSON files containing structured data with text entries. It extracts text entries
// from the JSON, sorts them based on numeric values in their filenames, and merges them into a single text file.
// The tool expects the JSON file structure to have entries with a "results" field, which contains an array of objects,
// each having a "text" field containing the actual text to be merged.
//
// Dependencies:
// - serde_json: For serializing and deserializing JSON data.
//
// How to Use:
// 1. Compile the code using the Rust compiler.
// 2. Run the executable with the following command-line arguments:
//    - The first argument: Path to the input JSON file.
//    - The second argument: Path to the output text file.
//
// Example Usage:
// $ ./json_text_merger input.json output.txt
//
// This tool provides a convenient way to merge text entries from JSON files, allowing for easy manipulation and
// processing of structured text data.
//

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <json_file> <output_file>", args[0]);
        return Ok(());
    }
    let json_file = &args[1];
    let output_file = &args[2];

    // Read the JSON file
    let mut file = File::open(json_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse JSON
    let json: BTreeMap<String, Value> = serde_json::from_str(&contents)?;

    // Create a vector to store the texts
    let mut texts: Vec<String> = Vec::new();

    // Iterate over each entry in the JSON
    for (_, value) in json.iter() {
        if let Some(results) = value.get("results") {
            if let Some(results_array) = results.as_array() {
                for result in results_array {
                    if let Some(text) = result.get("text") {
                        if let Some(text_str) = text.as_str() {
                            texts.push(text_str.to_string());
                        }
                    }
                }
            }
        }
    }

    // Sort texts based on the file name numbers
    texts.sort_by(|a, b| {
        let a_num: usize = a.split('_').nth(2).unwrap_or("0").parse().unwrap_or(0);
        let b_num: usize = b.split('_').nth(2).unwrap_or("0").parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });

    // Merge texts into a single string
    let merged_text = texts.join("\n");

    // Write merged text to the output file
    let mut output_file = File::create(output_file)?;
    output_file.write_all(merged_text.as_bytes())?;

    Ok(())
}