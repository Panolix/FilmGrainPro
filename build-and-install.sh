#!/bin/bash

# Film Grain Generator - Build and Install Script
# This script builds the app and installs it to Applications folder

set -e  # Exit on any error

echo "🎬 Film Grain Generator - Build & Install Script"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -d "film-grain-generator" ]; then
    echo -e "${RED}❌ Error: film-grain-generator directory not found!"
    echo -e "Please run this script from the project root directory.${NC}"
    exit 1
fi

echo -e "${BLUE}📁 Entering film-grain-generator directory...${NC}"
cd film-grain-generator

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js is not installed. Please install Node.js first.${NC}"
    exit 1
fi

# Check Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo is not installed. Please install Rust first.${NC}"
    exit 1
fi

# Check npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm is not installed. Please install npm first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ All prerequisites found!${NC}"

# Install dependencies
echo -e "${BLUE}📦 Installing dependencies...${NC}"
npm install

# Build the application
echo -e "${BLUE}🔨 Building Film Grain Generator...${NC}"
echo -e "${YELLOW}This may take a few minutes...${NC}"

npm run tauri build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi

# Find the built app
APP_PATH="src-tauri/target/release/bundle/macos/Film Grain Generator.app"
DMG_PATH="src-tauri/target/release/bundle/dmg/Film Grain Generator_0.0.0_aarch64.dmg"

if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}❌ Built app not found at expected location!${NC}"
    exit 1
fi

# Install to Applications folder
echo -e "${BLUE}📱 Installing to Applications folder...${NC}"

# Remove existing installation if it exists
if [ -d "/Applications/Film Grain Generator.app" ]; then
    echo -e "${YELLOW}🗑️  Removing existing installation...${NC}"
    rm -rf "/Applications/Film Grain Generator.app"
fi

# Copy new app to Applications
cp -R "$APP_PATH" "/Applications/"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Successfully installed Film Grain Generator to Applications!${NC}"
else
    echo -e "${RED}❌ Failed to install to Applications folder!${NC}"
    exit 1
fi

# Set proper permissions
echo -e "${BLUE}🔐 Setting proper permissions...${NC}"
chmod -R 755 "/Applications/Film Grain Generator.app"

# Show installation info
echo ""
echo -e "${GREEN}🎉 Installation Complete!${NC}"
echo "=================================="
echo -e "📱 App installed to: ${BLUE}/Applications/Film Grain Generator.app${NC}"
echo -e "💿 DMG available at: ${BLUE}$(pwd)/$DMG_PATH${NC}"
echo ""
echo -e "${YELLOW}🚀 You can now:${NC}"
echo "   • Launch from Applications folder"
echo "   • Launch from Spotlight (⌘+Space, type 'Film Grain')"
echo "   • Launch from Dock (if you add it)"
echo ""
echo -e "${BLUE}📊 App Features:${NC}"
echo "   • 40+ realistic film stocks with authentic characteristics"
echo "   • Multi-threaded grain rendering"
echo "   • Density slider (10K-50K grains)"
echo "   • Professional export (up to 2048x2048 PNG)"
echo "   • Film-specific colors, shapes, and clustering"
echo ""

# Ask if user wants to launch the app
echo -e "${YELLOW}Would you like to launch Film Grain Generator now? (y/n)${NC}"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo -e "${BLUE}🚀 Launching Film Grain Generator...${NC}"
    open "/Applications/Film Grain Generator.app"
fi

echo -e "${GREEN}✨ Enjoy creating authentic film grain overlays!${NC}"