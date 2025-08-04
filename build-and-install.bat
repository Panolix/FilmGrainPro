@echo off
REM Film Grain Generator - Build and Install Script for Windows
REM This script builds the app and installs it

echo.
echo 🎬 Film Grain Generator - Build ^& Install Script
echo ================================================

REM Check if we're in the right directory
if not exist "film-grain-generator" (
    echo ❌ Error: film-grain-generator directory not found!
    echo Please run this script from the project root directory.
    pause
    exit /b 1
)

echo 📁 Entering film-grain-generator directory...
cd film-grain-generator

REM Check prerequisites
echo 🔍 Checking prerequisites...

REM Check Node.js
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js is not installed. Please install Node.js first.
    pause
    exit /b 1
)

REM Check Rust/Cargo
cargo --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust/Cargo is not installed. Please install Rust first.
    pause
    exit /b 1
)

REM Check npm
npm --version >nul 2>&1
if errorlevel 1 (
    echo ❌ npm is not installed. Please install npm first.
    pause
    exit /b 1
)

echo ✅ All prerequisites found!

REM Install dependencies
echo 📦 Installing dependencies...
npm install

REM Build the application
echo 🔨 Building Film Grain Generator...
echo This may take a few minutes...

npm run tauri build

if errorlevel 1 (
    echo ❌ Build failed!
    pause
    exit /b 1
)

echo ✅ Build completed successfully!

REM Find the built app
set "APP_PATH=src-tauri\target\release\bundle\msi\Film Grain Generator_0.0.0_x64_en-US.msi"

if not exist "%APP_PATH%" (
    echo ❌ Built installer not found at expected location!
    echo Looking for: %APP_PATH%
    pause
    exit /b 1
)

echo.
echo 🎉 Build Complete!
echo ==================
echo 💿 Installer available at: %CD%\%APP_PATH%
echo.
echo 🚀 To install:
echo    1. Double-click the .msi file to run the installer
echo    2. Follow the installation wizard
echo    3. Launch from Start Menu or Desktop shortcut
echo.
echo 📊 App Features:
echo    • 40+ realistic film stocks with authentic characteristics
echo    • Multi-threaded grain rendering
echo    • Density slider (10K-50K grains)
echo    • Professional export (up to 2048x2048 PNG)
echo    • Film-specific colors, shapes, and clustering
echo.

REM Ask if user wants to run the installer
set /p response="Would you like to run the installer now? (y/n): "
if /i "%response%"=="y" (
    echo 🚀 Running installer...
    start "" "%APP_PATH%"
)

echo ✨ Enjoy creating authentic film grain overlays!
pause