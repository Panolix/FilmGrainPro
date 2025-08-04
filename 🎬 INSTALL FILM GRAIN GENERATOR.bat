@echo off
REM Universal Film Grain Generator Installer for Windows
REM This file can be double-clicked in Windows Explorer

title 🎬 Film Grain Generator - Universal Installer

REM Change to the directory where this script is located
cd /d "%~dp0"

echo.
echo 🎬 Film Grain Generator - Universal Installer
echo =============================================
echo.

REM Check if Python GUI installer exists
if exist "🎬 INSTALL FILM GRAIN GENERATOR.py" (
    echo 🚀 Starting GUI installer...
    echo.
    
    REM Try to run with Python
    python "🎬 INSTALL FILM GRAIN GENERATOR.py"
    
    if errorlevel 1 (
        echo.
        echo ❌ Failed to run Python installer!
        echo.
        echo Please make sure Python is installed from https://python.org
        echo Make sure to check "Add Python to PATH" during installation.
        echo.
        pause
        exit /b 1
    )
) else (
    echo ❌ GUI installer not found!
    echo.
    echo Please make sure "🎬 INSTALL FILM GRAIN GENERATOR.py" is in the same folder.
    echo.
    pause
    exit /b 1
)

REM Keep window open if there was an error
if errorlevel 1 (
    echo.
    pause
)