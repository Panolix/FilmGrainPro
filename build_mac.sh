#!/bin/bash

# FilmGrain Pro - macOS Build Script
echo "🎬 Building FilmGrain Pro for macOS..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This script is for macOS only"
    exit 1
fi

# Create build directory
mkdir -p build
cd build

# Check for dependencies
echo "📦 Checking dependencies..."

# Check for Homebrew
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found. Please install Homebrew first:"
    echo "   /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    exit 1
fi

# Install dependencies via Homebrew
echo "🔧 Installing dependencies..."
brew install cmake glfw nlohmann-json

# Optional: Install OpenCV for advanced image processing
read -p "📸 Install OpenCV for advanced image processing? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    brew install opencv
    echo "✅ OpenCV will be included"
else
    echo "⚠️  Using basic image processing (OpenCV skipped)"
fi

# Configure with CMake
echo "⚙️  Configuring build..."
cmake .. -DCMAKE_BUILD_TYPE=Release \
         -DCMAKE_OSX_DEPLOYMENT_TARGET=10.15 \
         -DHAVE_NLOHMANN_JSON=ON

if [ $? -ne 0 ]; then
    echo "❌ CMake configuration failed"
    exit 1
fi

# Build the project
echo "🔨 Building FilmGrain Pro..."
make -j$(sysctl -n hw.ncpu)

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build completed successfully!"
echo "🚀 Run the app with: ./bin/FilmGrainPro"

# Check if JSON files exist
if [ ! -f "bin/stocks5.json" ] || [ ! -f "bin/advanced-shapeetc.json" ]; then
    echo "⚠️  JSON data files not found in bin/ directory"
    echo "   Copying from source..."
    cp ../stocks5.json bin/ 2>/dev/null || echo "   stocks5.json not found"
    cp ../advanced-shapeetc.json bin/ 2>/dev/null || echo "   advanced-shapeetc.json not found"
fi

echo ""
echo "🎯 Next steps:"
echo "   1. cd build"
echo "   2. ./bin/FilmGrainPro"
echo ""
echo "📊 Your app will load 25 film stocks with scientific grain data!"