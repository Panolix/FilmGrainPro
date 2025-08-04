#!/bin/bash
# 🎬 Film Grain Generator - Simple Double-Click Installer for macOS
# Just double-click this file in Finder!

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Set terminal title
echo -e "\033]0;Film Grain Generator - Build & Install\007"

# Clear screen for clean look
clear

echo "🎬 Film Grain Generator - Build & Install"
echo "========================================"
echo ""
echo "This will build and install Film Grain Generator to your Applications folder."
echo ""
echo "📋 Requirements check:"

# Check Python 3
if command -v python3 &> /dev/null; then
    echo "✅ Python 3 found"
else
    echo "❌ Python 3 not found"
    echo ""
    echo "Please install Python 3 from https://python.org"
    echo "Then try running this installer again."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "build-and-install.py" ]; then
    echo "❌ build-and-install.py not found"
    echo ""
    echo "Please make sure this file is in the same folder as build-and-install.py"
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

echo "✅ Installer script found"
echo ""
echo "🚀 Ready to build and install!"
echo ""
echo "Press ENTER to continue or Ctrl+C to cancel..."
read

# Run the Python installer
python3 build-and-install.py

# Keep terminal open
echo ""
echo "🎉 Done! Press any key to close this window..."
read -n 1 -s