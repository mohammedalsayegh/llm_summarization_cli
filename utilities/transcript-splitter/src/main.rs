// Author: Mohammed H Alsaeygh
// Project: Transcript Splitter
//
// Description:
// This Rust utility splits transcript text files into smaller parts based on a maximum number of tokens per split. It reads a transcript file and wraps its content with configurable header and footer. If in single shot mode, it generates a single output file containing the entire transcript with the specified header and footer. If in split mode, it removes header lines, joins lines into paragraphs, and splits the text into smaller parts, each including the configured header and footer.
//
// How to Use:
// - Compile the code using the Rust compiler.
// - Run the executable with the following command-line arguments:
//   - -i <input_file>: Path to the input transcript file.
//   - -o <output_dir>: Optional. Output directory for split files. If not provided, splits will be saved in a directory named after the input file in the current directory.
//   - -s <max_tokens_per_split>: Maximum number of tokens (words) per split.
//   - -c <config_file>: Path to the configuration file specifying header and footer content.
//   - --single-shot: Optional flag to enable single shot mode, which generates a single output file for the entire transcript.
//
// Example Usage:
// $ ./transcript_splitter -i input.txt -o output_directory -s 1000 -c config.json
// $ ./transcript_splitter -i input.txt -c config.json --single-shot
//
// Dependencies:
// - serde: For JSON deserialization.
// - std: Standard Rust library for file I/O and command-line argument parsing.
//
// This tool simplifies the process of handling and processing transcript data, facilitating easier management and manipulation of large transcript files.

use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    header: String,
    footer: String,
}

fn read_config(config_file: &str) -> io::Result<Config> {
    let config_content = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}

fn wrap_with_header_footer(text: &str, config: &Config) -> String {
    format!("{}{}{}\n\n", config.header, text, config.footer)
}

fn single_shot_mode(
    input_file: &str,
    config_file: &str,
    output_dir: Option<&str>,
) -> io::Result<()> {
    let config = read_config(config_file)?;

    let file_content = fs::read_to_string(input_file)?;
    let wrapped_text = wrap_with_header_footer(&file_content, &config);

    let (file_name, file_extension) = split_extension(input_file);

    let output_dir = if let Some(dir) = output_dir {
    dir.to_string()
    } else {
        let current_dir = env::current_dir().unwrap();
        let file_stem = Path::new(&input_file).file_stem().unwrap().to_string_lossy();
        current_dir.join(format!("{}_splits", file_stem)).to_string_lossy().to_string()
    };

    fs::create_dir_all(&output_dir)?;

    let output_file = format!("{}/{}_single_shot{}", output_dir, file_name, file_extension);
    let mut output = File::create(output_file)?;
    output.write_all(wrapped_text.as_bytes())?;

    Ok(())
}

fn split_text(
    input_file: &str,
    max_tokens_per_split: usize,
    config_file: &str,
    output_dir: Option<&str>,
) -> io::Result<()> {
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Skip header lines (Start Time:, End Time:)
    let mut text_lines: Vec<String> = lines
        .into_iter()
        .filter(|line| !line.starts_with("Start Time:") && !line.starts_with("End Time:"))
        .map(|line| line.trim().to_string())
        .collect();

    // Keep "Script: " lines and remove only the prefix
    text_lines.iter_mut().for_each(|line| {
        if line.starts_with("Script: ") {
            *line = line.replacen("Script: ", "", 1);
        }
    });

    // Join the lines with spaces
    let text = text_lines.join(" ");
    let num_tokens = text.split_whitespace().count();
    let num_splits = (num_tokens - 1) / max_tokens_per_split + 1;

    let config = read_config(config_file)?;
    let (file_name, file_extension) = split_extension(input_file);

    // Determine the output directory
    let output_dir = if let Some(dir) = output_dir {
        dir.to_string()
    } else {
        let current_dir = env::current_dir().unwrap();
        let file_stem = Path::new(&input_file).file_stem().unwrap().to_string_lossy();
        current_dir.join(format!("{}_splits", file_stem)).to_string_lossy().to_string()
    };

    // Create the output directory
    fs::create_dir_all(&output_dir)?;

    for i in 0..num_splits {
        let start = i * max_tokens_per_split;
        let part_tokens: Vec<&str> = text.split_whitespace().skip(start).take(max_tokens_per_split).collect();
        let part_text = format!("{}{}{}\n\n", config.header, part_tokens.join(" "), config.footer);

        // Pad the index with zeros to ensure it has three digits
        let index_padded = format!("{:03}", i + 1);

        let output_file = format!("{}/{}_part_{}{}", output_dir, file_name, index_padded, file_extension);
        let mut output = File::create(output_file)?;
        output.write_all(part_text.as_bytes())?;
    }

    Ok(())
}

fn split_extension(file_path: &str) -> (String, String) {
    let path = Path::new(file_path);
    let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
    let extension = path.extension().unwrap().to_string_lossy().into_owned();
    (file_stem, format!(".{}", extension))
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let mut input_file = None;
    let mut output_dir = None;
    let mut max_tokens_per_split = None;
    let mut config_file = None;
    let mut single_shot = false; // Flag for single shot mode

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" => {
                if let Some(file) = args.next() {
                    input_file = Some(file);
                } else {
                    println!("Error: Expected input file after -i flag");
                    return Ok(());
                }
            }
            "-o" => {
                if let Some(dir) = args.next() {
                    output_dir = Some(dir);
                } else {
                    println!("Error: Expected output directory after -o flag");
                    return Ok(());
                }
            }
            "-s" => {
                if let Some(tokens) = args.next().and_then(|t| t.parse::<usize>().ok()) {
                    max_tokens_per_split = Some(tokens);
                } else {
                    println!("Error: Invalid value for max tokens per split");
                    return Ok(());
                }
            }
            "-c" => {
                if let Some(file) = args.next() {
                    config_file = Some(file);
                } else {
                    println!("Error: Expected config file after -c flag");
                    return Ok(());
                }
            }
            "--single-shot" => {
                single_shot = true; // Set the single shot flag
            }
            _ => {
                println!("Error: Invalid flag '{}'", arg);
                return Ok(());
            }
        }
    }

    if single_shot {
        let input_file = input_file.ok_or_else(|| {
            println!("Error: Missing input file argument (-i)");
            io::Error::from(io::ErrorKind::InvalidInput)
        })?;

        let config_file = config_file.ok_or_else(|| {
            println!("Error: Missing config file argument (-c)");
            io::Error::from(io::ErrorKind::InvalidInput)
        })?;

        let output_dir = output_dir.unwrap_or_else(|| {
            let current_dir = env::current_dir().unwrap();
            let file_stem = Path::new(&input_file).file_stem().unwrap().to_string_lossy();
            current_dir.join(format!("{}_splits", file_stem)).to_string_lossy().to_string()
        });

        single_shot_mode(&input_file, &config_file, Some(&output_dir))?;
    } else {
        let input_file = input_file.ok_or_else(|| {
            println!("Error: Missing input file argument (-i)");
            io::Error::from(io::ErrorKind::InvalidInput)
        })?;

        let output_dir = output_dir.unwrap_or_else(|| {
            let current_dir = env::current_dir().unwrap();
            let file_stem = Path::new(&input_file).file_stem().unwrap().to_string_lossy();
            current_dir.join(format!("{}_splits", file_stem)).to_string_lossy().to_string()
        });

        let max_tokens_per_split = max_tokens_per_split.ok_or_else(|| {
            println!("Error: Missing max tokens per split argument (-s)");
            io::Error::from(io::ErrorKind::InvalidInput)
        })?;

        let config_file = config_file.ok_or_else(|| {
            println!("Error: Missing config file argument (-c)");
            io::Error::from(io::ErrorKind::InvalidInput)
        })?;

        split_text(&input_file, max_tokens_per_split, &config_file, Some(&output_dir))?;
    }

    Ok(())
}