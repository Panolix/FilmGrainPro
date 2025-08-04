#!/bin/bash
# Universal Film Grain Generator Installer for Linux
# This file can be executed on Linux systems

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

echo "🎬 Film Grain Generator - Universal Installer"
echo "============================================="
echo ""

# Check if Python GUI installer exists
if [ -f "🎬 INSTALL FILM GRAIN GENERATOR.py" ]; then
    echo "🚀 Starting GUI installer..."
    echo ""
    
    # Try Python 3 first, then Python
    if command -v python3 &> /dev/null; then
        python3 "🎬 INSTALL FILM GRAIN GENERATOR.py"
    elif command -v python &> /dev/null; then
        python "🎬 INSTALL FILM GRAIN GENERATOR.py"
    else
        echo "❌ Python not found!"
        echo ""
        echo "Please install Python 3:"
        echo "  Ubuntu/Debian: sudo apt-get install python3 python3-tk"
        echo "  Fedora/RHEL:   sudo dnf install python3 python3-tkinter"
        echo "  Arch:          sudo pacman -S python python-tk"
        echo ""
        echo "Press any key to exit..."
        read -n 1 -s
        exit 1
    fi
else
    echo "❌ GUI installer not found!"
    echo ""
    echo "Please make sure '🎬 INSTALL FILM GRAIN GENERATOR.py' is in the same folder."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Keep terminal open if there was an error
if [ $? -ne 0 ]; then
    echo ""
    echo "Press any key to close..."
    read -n 1 -s
fi