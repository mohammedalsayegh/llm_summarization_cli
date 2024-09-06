@echo off
rem Start the timer
set start_time=%time%

:: Get the directory of the batch file
set "batch_dir=%~dp0"

set "full_path=.\"
set "filename=subtitles"
set "output_dir=%full_path%"

:: Remove temporary directories if they exist
if exist "%batch_dir%tmp" rd /s /q "%batch_dir%tmp"
if exist "%batch_dir%tmp_final" rd /s /q "%batch_dir%tmp_final"
:: del /S .\*.txt 2>NUL <- remove everthing in dir and sub_dir
del *.txt 2>NUL

:: Create temporary directory if it doesn't exist
mkdir "%batch_dir%tmp" 2>nul
set "tmp_dir=%batch_dir%tmp"
echo Directory '%tmp_dir%' created successfully.

:: Create final temporary directory if it doesn't exist
mkdir "%batch_dir%tmp_final" 2>nul
set "tmp_final_dir=%batch_dir%tmp_final"
echo Directory '%tmp_final_dir%' created successfully.

:: Run commands using PowerShell
echo this is the sub path: %1
powershell.exe -Command  ".\subtitle_file_to_txt.exe '%1'"
if not errorlevel 0 (
    echo Error: subtitle_file_to_txt.exe failed.
    exit /b 1
)
echo 'subtitle_file_to_txt.exe' completed successfully.

powershell.exe -Command ".\transcript-splitter.exe -i 'converted_subtitles.txt' -o '%tmp_dir%' -s 500 -c .\config_start.json"
if not errorlevel 0 (
    echo Error: transcript-splitter.exe failed.
    exit /b 1
)
echo transcript-splitter.exe completed successfully.

powershell.exe -Command "Remove-Item -Path '%filename%.txt' -ErrorAction SilentlyContinue"
echo File '%filename%.txt' removed successfully, if it existed.

powershell.exe -Command ".\ollama_summarization_cli.exe -d '%tmp_dir%' -o '%tmp_final_dir%\out.json' -u http://localhost:11434/api/generate" -m %2
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

powershell.exe -Command ".\ollama_summarization_cli.exe -d '%tmp_final_dir%' -o '%output_dir%out.json' -u http://localhost:11434/api/generate" -m %2
if not errorlevel 0 (
    echo Error: ollama_summarization_cli.exe for final step failed.
    exit /b 1
)
echo ollama_summarization_cli.exe for final step completed successfully.

powershell.exe -Command ".\json_text_merger.exe '%output_dir%out.json' '%output_dir%output_final.txt' ollama"
if not errorlevel 0 (
    echo Error: json_text_merger.exe for final step failed.
    exit /b 1
)
echo json_text_merger.exe for final step completed successfully.

:: Remove temporary directories
echo Cleaning up temporary files...
rmdir /s /q "%tmp_dir%"
rmdir /s /q "%tmp_final_dir%"

txt_youtube_id_rename.exe .\
echo Temporary files cleaned up successfully.

rem End the timer
set end_time=%time%

rem Calculate the time difference
set /a hours=(1%end_time:~0,2% - 100 - 1%start_time:~0,2% + 100) %% 24
set /a minutes=(1%end_time:~3,2% - 100 - 1%start_time:~3,2% + 100 + 60*hours) %% 60
set /a seconds=(1%end_time:~6,2% - 100 - 1%start_time:~6,2% + 100 + 60*minutes) %% 60

echo Execution Time: %hours% hours %minutes% minutes %seconds% seconds