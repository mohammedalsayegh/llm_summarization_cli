// Author: Mohammed H Alsaeygh
// Project: Ollama Text Summarization CLI Tool

// Description:
// This Rust command-line tool interfaces with the Ollama API, a Large Language Model (LLM) backend.
// It generates text summaries from large text files by sending requests to the Ollama API with pre-chunked text.
// The tool reads pre-chunked text from files in a specified directory, sends requests to the API, and stores the
// generated summaries in a JSON file. It provides a progress bar to visualize the processing status of each file.
// Only text files with the .txt extension are processed.
//
// Dependencies:
// - reqwest: For making HTTP requests to the Ollama API.
// - serde_json: For serializing and deserializing JSON data.
// - structopt: For parsing command-line arguments.
// - indicatif: For displaying progress bars.
//
// How to Use:
// 1. Compile the code using the Rust compiler.
// 2. Run the executable with the following command-line arguments:
//    -d or --dir: Specifies the directory containing pre-chunked text files.
//    -o or --output: Specifies the output JSON file.
//    -u or --url: Specifies the API URL for the Ollama API.
//    --params: Specifies the JSON file containing request parameters (optional).
//
// Example Usage:
// $ ./ollama_summarization_cli --dir /path/to/chunked_text_files --output output.json --url http://localhost:11434/api/generate
//
// This tool is designed for projects requiring automated text summarization of large text files by leveraging
// the capabilities of the Ollama API. It offers a straightforward approach to processing pre-chunked text
// and generating summaries efficiently.
//
// For more information about the Ollama API, visit: [https://github.com/ollama/ollama/blob/main/docs/api.md]

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "KoboldAI Summarization CLI Tool")]
struct CliArgs {
    #[structopt(
        short = "d",
        long = "dir",
        help = "Sets the directory containing text files"
    )]
    dir: String,

    #[structopt(short = "o", long = "output", help = "Sets the output JSON file")]
    output: String,

    #[structopt(
        short = "u",
        long = "url",
        help = "Sets the API URL for the KoboldAI API"
    )]
    api_url: String,

    #[structopt(
        short = "p",
        long = "params",
        help = "Sets the JSON file containing request parameters (optional)"
    )]
    params: Option<String>,
}

fn send_request(
    api_url: &str,
    prompt: &str,
    params: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let mut request_body = json!({
        "model": "phi3",
        "prompt": prompt.trim(),
        "stream": false
    });

    if let Some(params_path) = params {
        let params_json = fs::read_to_string(params_path)?;
        let params: Value = serde_json::from_str(&params_json)?;
        if let Value::Object(mut obj) = request_body {
            merge_json(&mut obj, &params);
            request_body = Value::Object(obj);
        }
    }

    let client = Client::new();
    let response = client
        .post(api_url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()?;

    if response.status().is_success() {
        let response_text = response.text()?;
        let response_json: Value = serde_json::from_str(&response_text)?;

        // Extract and return only the "response" field
        if let Some(response_value) = response_json.get("response") {
            if let Some(response_str) = response_value.as_str() {
                return Ok(response_str.to_string());
            }
        }

        // If "response" field is not found, return an error
        Err("No 'response' field found in JSON".into())
    } else {
        // If request fails, return error with status and response text
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::from_args();
    let mut results: HashMap<String, Value> = HashMap::new();

    let total_files = fs::read_dir(&args.dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|entry| {
                if let Some(ext) = entry.path().extension() {
                    if ext == "txt" {
                        Some(entry.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .count();

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
        match send_request(&args.api_url, &fs::read_to_string(&file_path)?, args.params.as_deref()) {
            Ok(response) => {
                // Tag the response with the filename and store in the hashmap
                results.insert(file_name.clone(), serde_json::Value::String(response));
            }
            Err(e) => {
                println!("Error processing {}: {}", file_name, e);
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message("All files processed.");

    let mut output_file = File::create(&args.output)?;
    serde_json::to_writer_pretty(&mut output_file, &results)?;

    Ok(())
}
