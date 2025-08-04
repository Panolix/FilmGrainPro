@echo off
REM Film Grain Generator - Windows Double-Click Installer
REM This file can be double-clicked in Windows Explorer to run the installer

title Film Grain Generator - Build and Install

REM Get the directory where this script is located
cd /d "%~dp0"

echo.
echo 🚀 Starting Film Grain Generator installer...
echo.

REM Check if Python is available
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python is required but not found.
    echo Please install Python from https://python.org and try again.
    echo.
    echo Make sure to check "Add Python to PATH" during installation.
    echo.
    pause
    exit /b 1
)

echo Press any key to continue or Ctrl+C to cancel...
pause >nul

REM Run the Python installer script
python build-and-install.py

REM Keep window open so user can see the results
echo.
echo Installation complete! Press any key to close this window...
pause >nul