#!/bin/bash
# Simple Command Line FilmGrainPro Installer
# This is a fallback installer that doesn't require GUI

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Set terminal title and clear screen
echo -e "\033]0;🎬 FilmGrainPro - Simple Installer\007"
clear

echo "🎬 FilmGrainPro - Simple Installer"
echo "=========================================="
echo ""
echo "This installer will build and install FilmGrainPro"
echo "without requiring a GUI interface."
echo ""

# Check prerequisites
echo "🔍 Checking prerequisites..."

# Check Python
if command -v python3 &> /dev/null; then
    echo "✅ Python 3 found"
elif command -v python &> /dev/null; then
    echo "✅ Python found"
else
    echo "❌ Python not found"
    echo ""
    echo "Please install Python 3 from https://python.org"
    echo "Then try running this installer again."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Check Node.js
if command -v node &> /dev/null; then
    echo "✅ Node.js found"
else
    echo "❌ Node.js not found"
    echo ""
    echo "Please install Node.js from https://nodejs.org"
    echo "Then try running this installer again."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Check Rust
if command -v cargo &> /dev/null; then
    echo "✅ Rust/Cargo found"
else
    echo "❌ Rust/Cargo not found"
    echo ""
    echo "Please install Rust from https://rustup.rs"
    echo "Then try running this installer again."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Check project structure
if [ ! -d "film-grain-generator" ]; then
    echo "❌ film-grain-generator directory not found"
    echo ""
    echo "Please make sure this installer is in the same folder"
    echo "as the film-grain-generator directory."
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

echo "✅ Project structure found"
echo ""

# Ask user to continue
echo "🚀 Ready to build and install FilmGrainPro!"
echo ""
echo "This process will:"
echo "• Install npm dependencies"
echo "• Build the Rust backend"
echo "• Compile the frontend"
echo "• Install the app to Applications folder"
echo ""
echo "This may take 5-10 minutes depending on your system."
echo ""
echo "Press ENTER to continue or Ctrl+C to cancel..."
read

# Start installation
echo ""
echo "📁 Entering project directory..."
cd film-grain-generator

echo ""
echo "📦 Installing npm dependencies..."
npm install

if [ $? -ne 0 ]; then
    echo "❌ Failed to install dependencies"
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

echo ""
echo "🔨 Building FilmGrainPro..."
echo "This will take several minutes..."
echo ""

npm run tauri build

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

echo ""
echo "📱 Installing to Applications folder..."

# Check if app was built
APP_PATH="src-tauri/target/release/bundle/macos/FilmGrainPro.app"
if [ ! -d "$APP_PATH" ]; then
    echo "❌ Built app not found at: $APP_PATH"
    echo ""
    echo "Press any key to exit..."
    read -n 1 -s
    exit 1
fi

# Remove existing installation
if [ -d "/Applications/FilmGrainPro.app" ]; then
    echo "🗑️  Removing existing installation..."
    rm -rf "/Applications/FilmGrainPro.app"
fi

# Copy new app
cp -R "$APP_PATH" "/Applications/"

if [ $? -eq 0 ]; then
    echo "✅ Successfully installed to Applications!"
    echo ""
    echo "🎉 Installation Complete!"
    echo "========================"
    echo ""
    echo "FilmGrainPro has been installed to:"
    echo "/Applications/FilmGrainPro.app"
    echo ""
    echo "You can now:"
    echo "• Launch from Applications folder"
    echo "• Search with Spotlight (⌘+Space)"
    echo "• Add to Dock for quick access"
    echo ""
    
    # Ask if user wants to launch
    echo "Would you like to launch FilmGrainPro now? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "🚀 Launching FilmGrainPro..."
        open "/Applications/FilmGrainPro.app"
    fi
    
    echo ""
    echo "✨ Enjoy creating authentic film grain overlays!"
else
    echo "❌ Failed to install to Applications folder"
    echo ""
    echo "You can still run the app from:"
    echo "$(pwd)/$APP_PATH"
fi

echo ""
echo "Press any key to close..."
read -n 1 -s