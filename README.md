Certainly! Here's how you can include the explanation of JSON parameters in the README:

```markdown
# koboldai_summarization_cli
KoboldAI Summarizer: Rust CLI tool for generating document summaries using KoboldAI's API. It facilitates the process of generating summaries from text files by interfacing with KoboldAI's language model backend.

## Features
- Summarize large text documents into concise summaries.
- Utilizes KoboldAI's powerful language model for accurate and coherent summarization.
- Provides a command-line interface for easy integration into workflows.
- Supports processing of text files in batches.
- Option to specify custom parameters for summarization.

## JSON Parameters
When using the CLI, you can specify custom parameters for the summarization process by providing a JSON file containing the following parameters:

1. **max_context_length**: Maximum length of the context (input) provided to the language model.
2. **max_length**: Maximum length of the generated summary.
3. **quiet**: Whether to suppress extra output from the API.
4. **rep_pen**: Repetition penalty discourages the model from repeating the same phrases or words in the generated summary.
5. **rep_pen_range**: Range of tokens over which the repetition penalty is applied.
6. **rep_pen_slope**: Slope of the repetition penalty function.
7. **temperature**: Controls the randomness of the sampling process during text generation.
8. **tfs**: Enables top-free sampling, allowing the model to sample from the entire vocabulary without restriction.
9. **top_a**: Number of highest-scoring tokens to keep for sampling in top-k sampling.
10. **top_k**: Value of k in top-k sampling, determining how many tokens to consider.
11. **top_p**: Threshold value for nucleus sampling, selecting tokens until the cumulative probability exceeds this value.
12. **typical**: Indicates whether to use "typical" sampling behavior.
13. **length_penalty**: Adjusts the length of the generated summary based on its length relative to the maximum length specified.

## Usage
1. **Installation**: Clone this repository and install the required dependencies using Cargo.
2. **Execution**: Run the program, specifying the directory containing text files, the output destination for the summaries, and optionally a JSON file containing custom parameters for summarization.

```sh
cargo build --release
./koboldai_summarization_cli --dir /path/to/text_files --output output.json [--params params.json]
```

## Dependencies
- `reqwest` for making HTTP requests to the KoboldAI API.
- `serde_json` for JSON serialization and deserialization.
- `indicatif` for progress bar functionality.
- `structopt` for command-line argument parsing.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
``` 

This README section provides an overview of the parameters that can be customized using JSON and how to use them with the CLI.