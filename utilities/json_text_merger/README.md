# JSON Text Merger

## Overview
JSON Text Merger is a command-line tool written in Rust that facilitates the merging of text entries from JSON files. This tool is designed to streamline the process of extracting text data from structured JSON files, sorting them based on numeric values in their filenames, and merging them into a single text file.

## Features
- **JSON Parsing**: Parses JSON files containing structured data with text entries.
- **Text Extraction**: Extracts text entries from the JSON files.
- **Sorting**: Sorts text entries based on numeric values in their filenames.
- **Merging**: Merges sorted text entries into a single text file.

## Usage
1. **Compile**: Compile the code using the Rust compiler.
2. **Run**: Execute the compiled binary with the following command-line arguments:
   - The first argument: Path to the input JSON file.
   - The second argument: Path to the output text file.

Example:
```bash
$ ./json_text_merger input.json output.txt
```

## Dependencies
- **serde_json**: For serializing and deserializing JSON data.

## Installation
To use JSON Text Merger, ensure you have Rust installed on your system. Then, clone the repository and compile the code using the following commands:
```bash
$ git clone <repository_url>
$ cd json-text-merger
$ cargo build --release
```
## Acknowledgements
JSON Text Merger makes use of the serde_json crate for JSON parsing. Special thanks to the Rust community for creating and maintaining such useful libraries.