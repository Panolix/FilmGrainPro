# 🎬 Film Grain Generator - Installation Guide

## 🚀 One-Click Universal Installer

### **Just Double-Click This File:**
```
🎬 INSTALL FILM GRAIN GENERATOR.py
```

**Works on all platforms:**
- 🍎 **macOS** (installs to Applications folder)
- 🪟 **Windows** (creates MSI installer and runs it)
- 🐧 **Linux** (creates AppImage or .deb package)

## ✨ What the Installer Does

### **Smart GUI Interface:**
- 📊 **Progress tracking** with real-time updates
- 📝 **Installation log** showing detailed progress
- 🔍 **Prerequisite checking** with helpful error messages
- 🎯 **Platform detection** and appropriate installation method

### **Automatic Process:**
1. **Checks prerequisites** (Node.js, Rust, npm)
2. **Installs dependencies** (npm packages)
3. **Builds the application** (Rust + frontend compilation)
4. **Installs to system** (Applications folder, Start Menu, etc.)
5. **Offers to launch** the app immediately

### **Cross-Platform Support:**
- **macOS**: Creates `.app` bundle → installs to `/Applications/`
- **Windows**: Creates `.msi` installer → runs automatically
- **Linux**: Creates `.AppImage` or `.deb` → installs appropriately

## 📋 Prerequisites

The installer will check for these automatically:

### **Required:**
- **Python 3.6+** (usually pre-installed on macOS/Linux)
- **Node.js 16+** ([Download](https://nodejs.org/))
- **Rust & Cargo** ([Install](https://rustup.rs/))

### **Quick Install Commands:**

**macOS:**
```bash
# Install Homebrew if needed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install requirements
brew install node
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
1. Download [Node.js](https://nodejs.org/) and install
2. Download [Rust](https://rustup.rs/) and install
3. Python is usually included with Windows 10/11

**Linux (Ubuntu/Debian):**
```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python (if needed)
sudo apt-get install python3 python3-tk
```

## 🔧 Alternative Installation Methods

### **Command Line (if GUI doesn't work):**
```bash
python3 build-and-install.py
```

### **Manual Build:**
```bash
cd film-grain-generator
npm install
npm run tauri build
```

## 🎯 What You Get

### **Professional Film Grain Generator:**
- **40+ Authentic Film Stocks** with realistic characteristics
- **Multi-threaded Rendering** using all CPU cores
- **Professional Controls** (density, intensity, size, opacity)
- **High-resolution Export** (up to 2048x2048 PNG)
- **Categorized Film Selection** (B&W, Color Negative, Slide)

### **Platform Integration:**
- **macOS**: Native `.app` in Applications folder
- **Windows**: Proper installer with Start Menu integration
- **Linux**: AppImage or system package installation

## 🐛 Troubleshooting

### **Common Issues:**

**"Prerequisites missing":**
- Install Node.js, Rust, and Python as shown above
- Restart terminal/command prompt after installation

**"Project not found":**
- Make sure the installer is in the same folder as `film-grain-generator/`
- Don't move the installer to a different location

**"Build failed":**
- Check internet connection (downloads dependencies)
- Ensure you have enough disk space (needs ~2GB during build)
- Try running `cargo clean` in `film-grain-generator/src-tauri/`

**"Permission denied" (macOS/Linux):**
- Run: `chmod +x "🎬 INSTALL FILM GRAIN GENERATOR.py"`
- Or use: `python3 "🎬 INSTALL FILM GRAIN GENERATOR.py"`

### **Getting Help:**
1. Check the installation log in the GUI for specific errors
2. Ensure all prerequisites are properly installed
3. Try the command line version if GUI fails
4. Check GitHub Issues for known problems

## 🎬 Ready to Create Film Grain!

Once installed, you'll have a professional film grain generator that creates **scientifically accurate** grain overlays based on real film stock characteristics. Perfect for:

- **Photography**: Add authentic film look to digital photos
- **Video/Cinema**: Match specific film stocks in post-production  
- **Design**: Create realistic textures for graphic design

**Enjoy creating authentic film grain overlays!** ✨