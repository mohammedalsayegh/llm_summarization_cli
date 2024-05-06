// Author: Mohammed H Alsaeygh
// Project: JSON Text Merger
//
// Description:
// This Rust command-line tool reads JSON files containing structured data with text entries. It extracts text entries
// from the JSON, sorts them based on numeric values in their filenames, and merges them into a single text file.
// The tool supports two modes of JSON file handling: "koboldai" mode and "ollama" mode. In "koboldai" mode, the tool
// expects the JSON file structure to have entries with a "results" field, which contains an array of objects,
// each having a "text" field containing the actual text to be merged. In "ollama" mode, the tool expects the JSON file
// structure to be a key-value pair where the keys represent filenames and the values represent the text content.
//
// Dependencies:
// - serde_json: For serializing and deserializing JSON data.
//
// How to Use:
// 1. Compile the code using the Rust compiler.
// 2. Run the executable with the following command-line arguments:
//    - The first argument: Path to the input JSON file.
//    - The second argument: Path to the output text file.
//    - The third argument: Mode of JSON file handling. Options are "koboldai" or "ollama".
//
// Example Usage:
// $ ./json_text_merger input.json output.txt koboldai
//
// This tool provides a convenient way to merge text entries from JSON files, allowing for easy manipulation and
// processing of structured text data.
//


use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
use serde::ser::Error;

enum JsonMode {
    Koboldai,
    Ollama,
}

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <json_file> <output_file> <mode>", args[0]);
        return Ok(());
    }
    let json_file = &args[1];
    let output_file = &args[2];
    let mode = match &args[3][..] {
        "koboldai" => JsonMode::Koboldai,
        "ollama" => JsonMode::Ollama,
        _ => {
            eprintln!("Invalid mode. Supported modes: koboldai, ollama");
            return Ok(());
        }
    };

    // Read the JSON file
    let mut file = File::open(json_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse JSON based on mode
    let texts = match mode {
        JsonMode::Koboldai => parse_koboldai_json(&contents),
        JsonMode::Ollama => parse_ollama_json(&contents),
    }?;

    // Sort texts based on the file name numbers
    let mut sorted_texts = texts.iter().collect::<Vec<_>>();
    sorted_texts.sort_by_key(|&(filename, _)| {
        filename
            .split('_')
            .nth(2)
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0)
    });

    // Merge texts into a single string
    let merged_text = sorted_texts
    .iter()
    .map(|&(_, ref text)| text.clone())
    .collect::<Vec<_>>()
    .join("\n");

    // Write merged text to the output file
    let mut output_file = File::create(output_file)?;
    output_file.write_all(merged_text.as_bytes())?;

    Ok(())
}

fn parse_koboldai_json(contents: &str) -> Result<Vec<(String, String)>, serde_json::Error> {
    let json: BTreeMap<String, Value> = serde_json::from_str(&contents)?;
    let mut texts = Vec::new();
    for (_, value) in json.iter() {
        if let Some(results) = value.get("results") {
            if let Some(results_array) = results.as_array() {
                for result in results_array {
                    if let Some(text) = result.get("text") {
                        if let Some(text_str) = text.as_str() {
                            texts.push(("".to_string(), text_str.to_string()));
                        }
                    }
                }
            }
        }
    }
    Ok(texts)
}

fn parse_ollama_json(contents: &str) -> Result<Vec<(String, String)>, serde_json::Error> {
    let json: BTreeMap<String, Value> = serde_json::from_str(&contents)?;
    let texts = json
        .iter()
        .map(|(filename, text)| {
            if let Some(text_str) = text.as_str() {
                Ok((filename.clone(), text_str.to_string()))
            } else {
                Err(serde_json::Error::custom("Invalid JSON format"))
            }
        })
        .collect::<Result<Vec<(String, String)>, serde_json::Error>>()?;
    Ok(texts)
}