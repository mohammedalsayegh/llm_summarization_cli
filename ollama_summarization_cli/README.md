# Ollama Text Summarization CLI Tool

This Rust command-line tool interfaces with the Ollama API, a Large Language Model (LLM) backend. It generates text summaries from large text files by sending requests to the Ollama API with pre-chunked text. The tool reads pre-chunked text from files in a specified directory, sends requests to the API, and stores the generated summaries in a JSON file. It provides a progress bar to visualize the processing status of each file. Only text files with the .txt extension are processed.

## Features
- Summarize large text documents into concise summaries.
- Utilizes Ollama's powerful language model for accurate and coherent summarization.
- Provides a command-line interface for easy integration into workflows.
- Supports processing of text files in batches.

## Usage
1. **Installation**: Clone this repository and install the required dependencies using Cargo.
2. **Execution**: Run the program, specifying the directory containing text files, the output destination for the summaries, the API URL, the model name, and optionally a JSON file containing custom parameters for summarization.

```sh
cargo build --release
./ollama_summarization_cli --dir /path/to/text_files --output output.json --url http://localhost:11434/api/generate --model phi3 [--params params.json]
```

## Dependencies
- `reqwest` for making HTTP requests to the Ollama API.
- `serde_json` for JSON serialization and deserialization.
- `indicatif` for progress bar functionality.
- `structopt` for command-line argument parsing.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.