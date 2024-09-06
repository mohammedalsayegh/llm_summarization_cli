# Subtitle File to TXT

This Rust utility converts `.srt` subtitle files into a custom transcript text format. It parses the subtitle files to extract script text along with start and end times in milliseconds, then writes this data to a text file.

## Features

- Converts `.srt` subtitle files to custom transcript text format
- Extracts script text and timestamps from `.srt` files
- Outputs the converted subtitles to a text file

## Usage

To convert a single `.srt` file:
```shell
$ ./subtitle_file_to_txt /path/to/subtitle.srt
```

The output will be saved as `converted_subtitles.txt` in the current directory.

## Dependencies

- regex: For parsing subtitle timestamps.
- std: Standard Rust library for file I/O and command-line argument parsing.