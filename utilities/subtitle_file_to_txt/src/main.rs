// Subtitle File to TXT
//
// Author: Mohammed H Alsaeygh
//
// Description:
// subtitle_file_to_txt is a Rust program designed to convert subtitles from .srt files into a custom text format.
// It takes the path of an .srt file as a command-line argument, parses the subtitle file, and writes the
// converted subtitles into a text file with timestamps.
//
// How to Use:
// 1. Compile the code using the Rust compiler.
// 2. Run the executable with a command-line argument representing the path to the .srt file.
//
// Example Usage:
// $ cargo run -- <path_to_srt_file>
//
// This program provides a convenient way to convert .srt subtitles into a more readable format for further analysis or use.

use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided the path to the .srt file as an argument
    if args.len() != 2 {
        println!("Usage: cargo run -- <path_to_srt_file>");
        return Ok(());
    }

    let srt_file_path = &args[1];

    // Open the .srt file
    let file = File::open(srt_file_path)?;
    let reader = BufReader::new(file);

    // Regular expression to match the subtitle time format
    let time_regex = Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3}) --> (\d{2}):(\d{2}):(\d{2}),(\d{3})").unwrap();

    // Variables to store parsed subtitle data
    let mut subtitles = String::new();
    let mut current_script = String::new();
    let mut start_ms = 0;
    let mut end_ms = 0;

    // Parse the .srt file
    for line in reader.lines() {
        let line = line?;

        // Check if the line matches the time format
        if let Some(captures) = time_regex.captures(&line) {
            if !current_script.is_empty() {
                // Add the current script to subtitles before starting a new one
                subtitles += &format!(
                    "Script: {}\nStart Time: {:?}\nEnd Time: {:?}\n\n",
                    current_script.trim(),
                    start_ms,
                    end_ms
                );
                current_script.clear();
            }

            // Parse start and end times in milliseconds
            start_ms = (captures[1].parse::<u64>().unwrap() * 3600 * 1000)
                + (captures[2].parse::<u64>().unwrap() * 60 * 1000)
                + (captures[3].parse::<u64>().unwrap() * 1000)
                + captures[4].parse::<u64>().unwrap();

            end_ms = (captures[5].parse::<u64>().unwrap() * 3600 * 1000)
                + (captures[6].parse::<u64>().unwrap() * 60 * 1000)
                + (captures[7].parse::<u64>().unwrap() * 1000)
                + captures[8].parse::<u64>().unwrap();
        } else if !line.trim().is_empty() && !line.chars().all(char::is_numeric) {
            // Collect subtitle script text (ignoring the index line and blank lines)
            current_script += &format!(" {}", line.trim());
        }
    }

    // Add the last script if any
    if !current_script.is_empty() {
        subtitles += &format!(
            "Script: {}\nStart Time: {:?}\nEnd Time: {:?}\n\n",
            current_script.trim(),
            start_ms,
            end_ms
        );
    }

    // Write the parsed subtitles to a text file
    let mut output_file = File::create("converted_subtitles.txt")?;
    output_file.write_all(subtitles.as_bytes())?;

    println!("Subtitles converted successfully.");
    Ok(())
}
