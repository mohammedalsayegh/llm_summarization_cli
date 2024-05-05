// Author: Mohammed H Alsaeygh
// Project: KoboldAI Text Summarization CLI Tool
//
// Description:
// This Rust command-line tool interfaces with the KoboldAI API, which handles the Large Language Model (LLM) backend.
// It generates text summaries from large text files by sending requests to the KoboldAI API with pre-chunked text.
// The tool reads pre-chunked text from files in a specified directory, sends requests to the API, and stores the
// generated summaries in a JSON file. It provides a progress bar to visualize the processing status of each file.
// Only text files with the .txt extension are processed.
//
// Dependencies:
// - reqwest: For making HTTP requests to the KoboldAI API.
// - serde_json: For serializing and deserializing JSON data.
// - structopt: For parsing command-line arguments.
// - indicatif: For displaying progress bars.
//
// How to Use:
// 1. Compile the code using the Rust compiler.
// 2. Run the executable with the following command-line arguments:
//    -d or --dir: Specifies the directory containing pre-chunked text files.
//    -o or --output: Specifies the output JSON file.
//    --params: Specifies the JSON file containing request parameters (optional).
//
// Example Usage:
// $ ./koboldai_summarization_cli --dir /path/to/chunked_text_files --output output.json
//
// This tool is designed for projects requiring automated text summarization of large text files by leveraging
// the capabilities of the KoboldAI API. It offers a straightforward approach to processing pre-chunked text
// and generating summaries efficiently.
//
// For more information about the KoboldAI API, visit: [https://lite.koboldai.net/koboldcpp_api#]

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::{collections::HashMap, fs};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "My CLI")]
struct CliArgs {
    #[structopt(
        short = "d",
        long = "dir",
        help = "Sets the directory containing text files"
    )]
    dir: String,

    #[structopt(short = "o", long = "output", help = "Sets the output JSON file")]
    output: String, // New argument to specify the output JSON file

    #[structopt(
        short = "p",
        long = "params",
        help = "Sets the JSON file containing request parameters (optional)"
    )]
    params: Option<String>, // Optional argument to specify the parameters JSON file
}

// Send request to the API
fn send_request(
    txt_file_path: &str,
    params: Option<&str>,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Read the prompt from a text file
    let prompt = fs::read_to_string(txt_file_path)?;

    // Default request parameters
    let mut request_body = json!({
        "max_context_length": 512,
        "max_length": 100,
        "prompt": prompt.trim(),
        "quiet": false,
        "rep_pen": 1.1,
        "rep_pen_range": 256,
        "rep_pen_slope": 1,
        "temperature": 0.5,
    });

    // If params file path is provided, merge parameters from the file
    if let Some(params_path) = params {
        let params_json = fs::read_to_string(params_path)?;
        let params: Value = serde_json::from_str(&params_json)?;

        // Ensure that request_body is a mutable reference
        if let Value::Object(mut obj) = request_body {
            // Merge parameters from the file into default parameters
            merge_json(&mut obj, &params);
            request_body = Value::Object(obj); // Convert back to Value
        }
    }

    // Send the request
    let client = Client::new();
    let response = client
        .post("http://localhost:5001/api/v1/generate")
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()?;

    // Check if the request was successful
    if response.status().is_success() {
        // Extract the response body as JSON
        let response_json: Value = response.json()?; // Parse JSON response
        Ok(response_json)
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

// Function to merge JSON objects
fn merge_json(base: &mut serde_json::Map<String, Value>, new: &Value) {
    if let Value::Object(new_obj) = new {
        for (key, value) in new_obj.iter() {
            if let Some(base_value) = base.get_mut(key) {
                if let Value::Object(base_obj) = base_value {
                    merge_json(base_obj, value);
                }
            } else {
                base.insert(key.clone(), value.clone());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = CliArgs::from_args();

    // Create a hashmap to store results tagged by filename
    let mut results: HashMap<String, Value> = HashMap::new();

    // Count the total number of files
    let total_files = fs::read_dir(&args.dir)?
        .filter(|entry| {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    return ext == "txt";
                }
            }
            false
        })
        .count();

    // Create a progress bar
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")?
            .progress_chars("#>-"),
    );

    // Iterate over all text files in the directory
    for entry in fs::read_dir(&args.dir)? {
        let entry = entry?;
        let file_path = entry.path();

        if let Some(ext) = file_path.extension() {
            if ext != "txt" {
                // Skip non-text files
                continue;
            }
        } else {
            // Skip directories and files without extensions
            continue;
        }

        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        // Send request for each file
        match send_request(file_path.to_str().unwrap(), args.params.as_deref()) {
            Ok(response) => {
                // Tag the response with the filename and store in the hashmap
                results.insert(file_name.clone(), response);
            }
            Err(e) => {
                println!("Error processing {}: {}", file_name, e);
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("All files processed.");

    // Write all results to the output JSON file
    let mut output_file = fs::File::create(&args.output)?;
    serde_json::to_writer_pretty(&mut output_file, &results)?;

    Ok(())
}