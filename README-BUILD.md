# 🎬 Film Grain Generator - Build & Install Guide

## 🚀 Quick Start (Cross-Platform)

### **One-Command Build & Install:**

```bash
# On macOS/Linux:
python3 build-and-install.py

# On Windows:
python build-and-install.py
```

## 📋 Prerequisites

Before running the build script, make sure you have:

### **Required Software:**
- **Python 3.6+** (usually pre-installed on macOS/Linux)
- **Node.js 16+** ([Download](https://nodejs.org/))
- **Rust & Cargo** ([Install](https://rustup.rs/))
- **npm** (comes with Node.js)

### **Quick Install Commands:**

**macOS (using Homebrew):**
```bash
# Install Homebrew if you don't have it
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install required tools
brew install node
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
1. Download and install [Node.js](https://nodejs.org/)
2. Download and install [Rust](https://rustup.rs/)
3. Python is usually included with Windows 10/11

**Linux (Ubuntu/Debian):**
```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python (if not already installed)
sudo apt-get install python3
```

## 🔧 What the Script Does

### **1. Prerequisites Check:**
- ✅ Verifies all required tools are installed
- ❌ Shows clear error messages for missing dependencies

### **2. Automatic Build:**
- 📦 Installs npm dependencies
- 🔨 Compiles the Rust backend
- 🎨 Builds the frontend
- 📱 Creates platform-specific installers

### **3. Smart Installation:**
- **macOS**: Installs `.app` to `/Applications/` folder
- **Windows**: Creates and optionally runs `.msi` installer
- 🗑️ Removes old installations automatically
- 🔐 Sets proper permissions

### **4. Platform Detection:**
- 🍎 Automatically detects macOS and uses `.app` bundle
- 🪟 Automatically detects Windows and uses `.msi` installer
- 🐧 Shows helpful message for Linux users

## 🎯 Manual Build (Alternative)

If you prefer to build manually:

```bash
cd film-grain-generator

# Install dependencies
npm install

# Build for production
npm run tauri build

# Find your built app:
# macOS: src-tauri/target/release/bundle/macos/Film Grain Generator.app
# Windows: src-tauri/target/release/bundle/msi/Film Grain Generator_*.msi
```

## 🐛 Troubleshooting

### **Common Issues:**

**"Command not found" errors:**
- Make sure all prerequisites are installed
- Restart your terminal after installing new tools
- On Windows, you may need to restart your computer

**Build fails with Rust errors:**
- Update Rust: `rustup update`
- Clear cache: `cargo clean`

**Permission denied on macOS:**
- Run: `chmod +x build-and-install.py`
- Or use: `python3 build-and-install.py`

**Windows Defender blocks the installer:**
- This is normal for unsigned apps
- Click "More info" → "Run anyway"

### **Getting Help:**

1. Check that all prerequisites are properly installed
2. Try the manual build process
3. Check the terminal output for specific error messages
4. Make sure you're running from the correct directory

## 🎬 App Features

Once installed, you'll have access to:

- **40+ Realistic Film Stocks** with authentic characteristics
- **Multi-threaded Rendering** using all CPU cores
- **Density Control** (10K-50K grains)
- **Professional Export** (up to 2048x2048 PNG)
- **Film-specific Colors, Shapes, and Clustering**
- **Real-time Preview** with instant updates

## 📝 Development

For development builds:

```bash
cd film-grain-generator
npm run tauri dev
```

This starts the app in development mode with hot-reload.

---

**Made with ❤️ for film photography enthusiasts and digital artists**