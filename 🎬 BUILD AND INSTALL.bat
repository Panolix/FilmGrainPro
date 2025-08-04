@echo off
REM 🎬 Film Grain Generator - Simple Double-Click Installer for Windows
REM Just double-click this file in Windows Explorer!

title 🎬 Film Grain Generator - Build and Install

REM Change to the directory where this script is located
cd /d "%~dp0"

echo.
echo 🎬 Film Grain Generator - Build and Install
echo ========================================
echo.
echo This will build and install Film Grain Generator on your computer.
echo.
echo 📋 Requirements check:

REM Check Python
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python not found
    echo.
    echo Please install Python from https://python.org
    echo Make sure to check "Add Python to PATH" during installation.
    echo Then try running this installer again.
    echo.
    pause
    exit /b 1
) else (
    echo ✅ Python found
)

REM Check if installer script exists
if not exist "build-and-install.py" (
    echo ❌ build-and-install.py not found
    echo.
    echo Please make sure this file is in the same folder as build-and-install.py
    echo.
    pause
    exit /b 1
) else (
    echo ✅ Installer script found
)

echo.
echo 🚀 Ready to build and install!
echo.
echo Press any key to continue or Ctrl+C to cancel...
pause >nul

REM Run the Python installer
python build-and-install.py

REM Keep window open
echo.
echo 🎉 Done! Press any key to close this window...
pause >nul