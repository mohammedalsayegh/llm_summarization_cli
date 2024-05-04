# Transcript Splitter

Transcript Splitter is a Rust utility designed to split transcript text files into smaller parts based on a maximum number of tokens per split. It aims to simplify the process of handling and processing large transcript files by breaking them down into more manageable chunks.

## Features

- Splits transcript text files into smaller parts.
- Configurable header and footer for each split.
- Supports specifying the maximum number of tokens (words) per split.
- Easy-to-use command-line interface.

## How to Use

1. **Compilation:**
   - Ensure you have Rust installed.
   - Clone this repository to your local machine.
   - Navigate to the project directory in your terminal.
   - Run `cargo build --release` to compile the project.

2. **Execution:**
   - After successful compilation, you can find the executable in the `target/release` directory.
   - Run the executable with the following command-line arguments:
     ```
     ./transcript_splitter -i <input_file> -o <output_dir> -s <max_tokens_per_split> -c <config_file>
     ```
     - `-i <input_file>`: Path to the input transcript file.
     - `-o <output_dir>`: Optional. Output directory for split files. If not provided, splits will be saved in a directory named after the input file in the current directory.
     - `-s <max_tokens_per_split>`: Maximum number of tokens (words) per split.
     - `-c <config_file>`: Path to the configuration file specifying header and footer content.

3. **Example Usage:**
   ```
   ./transcript_splitter -i input.txt -o output_directory -s 1000 -c config.json
   ```

## Configuration

Transcript Splitter requires a configuration file specifying the header and footer content for each split. The configuration file must be in JSON format and should contain the following fields:
```json
{
  "header": "Header content here",
  "footer": "Footer content here"
}
```

## Dependencies

- serde: For JSON deserialization.
- std: Standard Rust library for file I/O and command-line argument parsing.