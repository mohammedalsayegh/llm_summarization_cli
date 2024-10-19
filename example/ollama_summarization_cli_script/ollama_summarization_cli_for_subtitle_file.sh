#!/bin/bash

# Start the timer
start_time=$(date +%s)

# Get the directory of the script
batch_dir="$(dirname "$0")/"

full_path="./"
filename="subtitles"
output_dir="$full_path"
model_name="llama3.1"

# Remove temporary directories if they exist
rm -rf "${batch_dir}tmp" "${batch_dir}tmp_final"

# Remove .txt and .srt files in the current directory
rm -f *.txt *.srt

# Create temporary directory if it doesn't exist
mkdir -p "${batch_dir}tmp"
tmp_dir="${batch_dir}tmp"
echo "Directory '${tmp_dir}' created successfully."

# Create final temporary directory if it doesn't exist
mkdir -p "${batch_dir}tmp_final"
tmp_final_dir="${batch_dir}tmp_final"
echo "Directory '${tmp_final_dir}' created successfully."

# Run commands using Bash
echo "This is the sub path: $1"
./subtitle_file_to_txt "$1"
if [ $? -ne 0 ]; then
    echo "Error: subtitle_file_to_txt failed."
    exit 1
fi
echo "subtitle_file_to_txt completed successfully."

./transcript-splitter -i "converted_subtitles.txt" -o "$tmp_dir" -s 500 -c "./config_start.json"
if [ $? -ne 0 ]; then
    echo "Error: transcript-splitter failed."
    exit 1
fi
echo "transcript-splitter completed successfully."

rm -f "${filename}.txt"
echo "File '${filename}.txt' removed successfully, if it existed."

./ollama_summarization_cli -d "$tmp_dir" -o "${tmp_final_dir}/out.json" -u "http://localhost:11434/api/generate" -m "$model_name"
if [ $? -ne 0 ]; then
    echo "Error: ollama_summarization_cli failed."
    exit 1
fi
echo "ollama_summarization_cli completed successfully."

./json_text_merger "${tmp_final_dir}/out.json" "${tmp_final_dir}/${filename}_output.txt" "ollama"
if [ $? -ne 0 ]; then
    echo "Error: json_text_merger failed."
    exit 1
fi
echo "json_text_merger completed successfully."

echo "DONE MAJOR!"

./transcript-splitter -i "${tmp_final_dir}/${filename}_output.txt" -o "$tmp_final_dir" -c "./config_final.json" --single-shot
if [ $? -ne 0 ]; then
    echo "Error: transcript-splitter for final step failed."
    exit 1
fi
echo "transcript-splitter for final step completed successfully."

if [ -f "${tmp_final_dir}/${filename}_output.txt" ]; then
    rm "${tmp_final_dir}/${filename}_output.txt"
    echo "File '${tmp_final_dir}/${filename}_output.txt' deleted successfully."
fi

./ollama_summarization_cli -d "$tmp_final_dir" -o "${output_dir}/out.json" -u "http://localhost:11434/api/generate" -m "$model_name"
if [ $? -ne 0 ]; then
    echo "Error: ollama_summarization_cli for final step failed."
    exit 1
fi
echo "ollama_summarization_cli for final step completed successfully."

rm -f "${batch_dir}*.txt" || echo "Error: Failed to delete .txt files."
rm -f "${batch_dir}*.srt" || echo "Error: Failed to delete .srt files."

./json_text_merger "${output_dir}/out.json" "${output_dir}/output_$(basename "$1" .srt).txt" "ollama"
if [ $? -ne 0 ]; then
    echo "Error: json_text_merger for final step failed."
    exit 1
fi
echo "json_text_merger for final step completed successfully."

# Remove temporary directories
echo "Cleaning up temporary files..."
rm -rf "$tmp_dir" "$tmp_final_dir"
echo "Temporary files cleaned up successfully."

# End the timer
end_time=$(date +%s)

# Calculate the time difference
elapsed_time=$((end_time - start_time))
hours=$((elapsed_time / 3600))
minutes=$(( (elapsed_time % 3600) / 60 ))
seconds=$((elapsed_time % 60))

echo "Execution Time: $hours hours $minutes minutes $seconds seconds"