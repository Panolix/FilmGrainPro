#!/bin/bash
# Film Grain Generator - macOS Double-Click Installer
# This file can be double-clicked in Finder to run the installer

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Clear the terminal for a clean look
clear

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    echo "🚀 Starting Film Grain Generator installer..."
    echo "Press any key to continue or Ctrl+C to cancel..."
    read -n 1 -s
    python3 build-and-install.py
else
    echo "❌ Python 3 is required but not found."
    echo "Please install Python 3 and try again."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
fi

# Keep terminal open so user can see the results
echo ""
echo "Press any key to close this window..."
read -n 1 -s