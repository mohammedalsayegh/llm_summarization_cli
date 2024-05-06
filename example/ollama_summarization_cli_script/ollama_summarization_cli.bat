@echo off

:: Warning message
echo WARNING: This script may perform operations on your files that are not fully revised. Do you want to continue? (yes/no)
set /p choice=Choice: 

:: Check user choice
if /i "%choice%"=="no" (
    echo Aborted by user.
    exit /b 1
) else if /i "%choice%"=="yes" (
    echo Continuing...
) else (
    echo Invalid choice. Please choose 'yes' or 'no'.
    exit /b 1
)

if "%~1"=="" (
    echo Usage: %0 "full_path\filename"
    exit /b 1
)

:: Get the directory of the batch file
set "batch_dir=%~dp0"

set "full_path=%~dp1"
set "filename=%~n1"
set "output_dir=%full_path%"

:: Remove temporary directories if they exist
if exist "%batch_dir%tmp" rd /s /q "%batch_dir%tmp"
if exist "%batch_dir%tmp_final" rd /s /q "%batch_dir%tmp_final"

:: Create temporary directory if it doesn't exist
mkdir "%batch_dir%tmp" 2>nul
set "tmp_dir=%batch_dir%tmp"
echo Directory '%tmp_dir%' created successfully.

:: Create final temporary directory if it doesn't exist
mkdir "%batch_dir%tmp_final" 2>nul
set "tmp_final_dir=%batch_dir%tmp_final"
echo Directory '%tmp_final_dir%' created successfully.

:: Run commands using PowerShell
powershell.exe -Command ".\noscribe_transcript_extractor.exe '%full_path%%filename%.html' '%tmp_dir%'"
if not errorlevel 0 (
    echo Error: noscribe_transcript_extractor.exe failed.
    exit /b 1
)
echo noscribe_transcript_extractor.exe completed successfully.

powershell.exe -Command ".\transcript-splitter.exe -i '%tmp_dir%\%filename%.txt' -o '%tmp_dir%' -s 500 -c .\config_start.json"
if not errorlevel 0 (
    echo Error: transcript-splitter.exe failed.
    exit /b 1
)
echo transcript-splitter.exe completed successfully.

powershell.exe -Command "Remove-Item -Path '%tmp_dir%\%filename%.txt' -ErrorAction SilentlyContinue"
echo File '%tmp_dir%\%filename%.txt' removed successfully, if it existed.

powershell.exe -Command ".\ollama_summarization_cli.exe -d '%tmp_dir%' -o '%tmp_final_dir%\out.json' -u http://localhost:11434/api/generate"
if not errorlevel 0 (
    echo Error: ollama_summarization_cli.exe failed.
    exit /b 1
)
echo ollama_summarization_cli.exe completed successfully.

powershell.exe -Command ".\json_text_merger.exe '%tmp_final_dir%\out.json' '%tmp_final_dir%\%filename%_output.txt' ollama"
if not errorlevel 0 (
    echo Error: json_text_merger.exe failed.
    exit /b 1
)
echo json_text_merger.exe completed successfully.

echo "DONE MAJOR!"

powershell.exe -Command ".\transcript-splitter.exe -i '%tmp_final_dir%\%filename%_output.txt' -o '%tmp_final_dir%' -c .\config_final.json --single-shot"

if not errorlevel 0 (
    echo Error: transcript-splitter.exe for final step failed.
    exit /b 1
)
echo transcript-splitter.exe for final step completed successfully.

:: Delete the file '%tmp_final_dir%\%filename%'
if exist "%tmp_final_dir%\%filename%_output.txt" (
    del "%tmp_final_dir%\%filename%_output.txt"
    echo File '%tmp_final_dir%\%filename%_output.txt' deleted successfully.
)

powershell.exe -Command ".\ollama_summarization_cli.exe -d '%tmp_final_dir%' -o '%output_dir%out.json' -u http://localhost:11434/api/generate"
if not errorlevel 0 (
    echo Error: ollama_summarization_cli.exe for final step failed.
    exit /b 1
)
echo ollama_summarization_cli.exe for final step completed successfully.

powershell.exe -Command ".\json_text_merger.exe '%output_dir%out.json' '%output_dir%%filename%_output.txt' ollama"
if not errorlevel 0 (
    echo Error: json_text_merger.exe for final step failed.
    exit /b 1
)
echo json_text_merger.exe for final step completed successfully.

:: Remove temporary directories
echo Cleaning up temporary files...
rmdir /s /q "%tmp_dir%"
rmdir /s /q "%tmp_final_dir%"

echo Temporary files cleaned up successfully.

pause