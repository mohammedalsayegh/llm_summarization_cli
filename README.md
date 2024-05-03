# koboldai_summarization_cli
KoboldAI Summarizer: Rust CLI tool for generating document summaries using KoboldAI's API. It facilitates the process of generating summaries from text files by interfacing with KoboldAI's language model backend.

## Features
- Summarize large text documents into concise summaries.
- Utilizes KoboldAI's powerful language model for accurate and coherent summarization.
- Provides a command-line interface for easy integration into workflows.
- Supports processing of text files in batches.

## Usage
1. **Installation**: Clone this repository and install the required dependencies using Cargo.
2. **Execution**: Run the program, specifying the directory containing text files and the output destination for the summaries.

## Dependencies
- `reqwest` for making HTTP requests to the KoboldAI API.
- `serde_json` for JSON serialization and deserialization.
- `indicatif` for progress bar functionality.
- `structopt` for command-line argument parsing.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.