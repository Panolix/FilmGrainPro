#!/bin/bash
# Universal Film Grain Generator Installer
# This file can be double-clicked on macOS to run the Python installer

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Set terminal title
echo -e "\033]0;🎬 Film Grain Generator Installer\007"

# Clear screen
clear

echo "🎬 Film Grain Generator - Universal Installer"
echo "============================================="
echo ""

# Check if Python GUI installer exists
if [ -f "🎬 INSTALL FILM GRAIN GENERATOR.py" ]; then
    echo "🚀 Starting installer..."
    echo ""
    
    # Try Python 3 first, then Python
    if command -v python3 &> /dev/null; then
        echo "Attempting to start GUI installer..."
        python3 "🎬 INSTALL FILM GRAIN GENERATOR.py" 2>&1
        
        # If GUI fails, fall back to command line
        if [ $? -ne 0 ]; then
            echo ""
            echo "⚠️  GUI installer failed. Starting command line installer..."
            echo ""
            
            # Check if fallback installer exists
            if [ -f "build-and-install.py" ]; then
                python3 "build-and-install.py"
            else
                echo "❌ No fallback installer found."
                echo ""
                echo "Manual installation steps:"
                echo "1. cd film-grain-generator"
                echo "2. npm install"
                echo "3. npm run tauri build"
                echo ""
                echo "Press any key to exit..."
                read -n 1 -s
            fi
        fi
    elif command -v python &> /dev/null; then
        echo "Attempting to start GUI installer..."
        python "🎬 INSTALL FILM GRAIN GENERATOR.py" 2>&1
        
        # If GUI fails, fall back to command line
        if [ $? -ne 0 ]; then
            echo ""
            echo "⚠️  GUI installer failed. Starting command line installer..."
            echo ""
            
            # Check if fallback installer exists
            if [ -f "build-and-install.py" ]; then
                python "build-and-install.py"
            else
                echo "❌ No fallback installer found."
                echo ""
                echo "Manual installation steps:"
                echo "1. cd film-grain-generator"
                echo "2. npm install"
                echo "3. npm run tauri build"
                echo ""
                echo "Press any key to exit..."
                read -n 1 -s
            fi
        fi
    else
        echo "❌ Python not found!"
        echo ""
        echo "Please install Python 3 from https://python.org"
        echo "Then try running this installer again."
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