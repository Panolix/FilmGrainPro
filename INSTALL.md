# Installation Guide

This guide covers building Film Grain Generator from source code.

## Prerequisites

### Required Software

#### Rust & Cargo
```bash
# Install Rust (includes Cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
cargo --version
rustc --version
```

#### Node.js & npm
```bash
# macOS (using Homebrew)
brew install node

# Windows (download from nodejs.org)
# Or use package manager of choice

# Verify installation
node --version
npm --version
```

### Platform-Specific Requirements

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install additional dependencies
brew install pkg-config
```

#### Windows
```bash
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

# Or install Visual Studio Community with C++ workload
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev
```

## Building from Source

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/film-grain-generator.git
cd film-grain-generator
```

### 2. Install Dependencies
```bash
# Install Node.js dependencies
npm install

# Install Tauri CLI
cargo install tauri-cli --locked
```

### 3. Build the Application

#### Development Build (Fast)
```bash
# Build and run in development mode
cargo tauri dev
```

#### Production Build
```bash
# Build optimized release version
npm run build
cargo tauri build
```

### 4. Locate Built Application

#### macOS
```bash
# Application bundle
./src-tauri/target/release/bundle/macos/Film\ Grain\ Generator.app

# DMG installer
./src-tauri/target/release/bundle/dmg/Film\ Grain\ Generator_*.dmg
```

#### Windows
```bash
# Executable
./src-tauri/target/release/Film\ Grain\ Generator.exe

# MSI installer
./src-tauri/target/release/bundle/msi/Film\ Grain\ Generator_*.msi
```

#### Linux
```bash
# Executable
./src-tauri/target/release/film-grain-generator

# AppImage
./src-tauri/target/release/bundle/appimage/film-grain-generator_*.AppImage

# DEB package
./src-tauri/target/release/bundle/deb/film-grain-generator_*.deb
```

## Development Setup

### Project Structure
```
film-grain-generator/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   └── main.rs      # Main application logic
│   ├── Cargo.toml       # Rust dependencies
│   └── tauri.conf.json  # Tauri configuration
├── index.html           # Frontend HTML
├── main.js             # Frontend JavaScript
├── package.json        # Node.js dependencies
├── fixed.json          # Film stock data
├── color.json          # Grain color data
└── README.md
```

### Development Commands

#### Start Development Server
```bash
# Hot-reload development mode
cargo tauri dev
```

#### Build Frontend Only
```bash
npm run build
```

#### Check Rust Code
```bash
cd src-tauri
cargo check
cargo clippy  # Linting
cargo fmt     # Formatting
```

#### Run Tests
```bash
cd src-tauri
cargo test
```

## Troubleshooting

### Common Issues

#### "cargo: command not found"
```bash
# Ensure Rust is installed and in PATH
source ~/.cargo/env
# Or restart your terminal
```

#### "tauri: command not found"
```bash
# Install Tauri CLI
cargo install tauri-cli --locked

# Ensure ~/.cargo/bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

#### Build Fails on macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Update Rust
rustup update
```

#### Build Fails on Windows
```bash
# Ensure Visual Studio Build Tools are installed
# Restart terminal after installation
# Try building again
```

#### "Failed to load film stock data"
```bash
# Ensure data files are present
ls fixed.json color.json

# If missing, they should be in the repository root
```

### Performance Issues

#### Slow Build Times
```bash
# Use more CPU cores for compilation
export CARGO_BUILD_JOBS=8  # Adjust for your CPU

# Use faster linker (macOS)
brew install michaeleisel/zld/zld
```

#### Runtime Performance
- Ensure you're using a release build for testing
- Lower max grain density for older hardware
- Close other applications during grain generation

### Platform-Specific Notes

#### macOS Code Signing
For distribution, you'll need to sign the application:
```bash
# Sign the app bundle (requires Apple Developer account)
codesign --force --deep --sign "Developer ID Application: Your Name" \
  "./src-tauri/target/release/bundle/macos/Film Grain Generator.app"
```

#### Windows Antivirus
Some antivirus software may flag the executable. This is common with Rust applications. You may need to add an exception.

## Contributing to Development

### Setting Up Development Environment
1. Follow the build instructions above
2. Install recommended VS Code extensions:
   - rust-analyzer
   - Tauri
   - ES6 String HTML
3. Configure your editor for Rust development

### Making Changes
1. Create a feature branch
2. Make your changes
3. Test thoroughly with `cargo tauri dev`
4. Build release version to verify
5. Submit a pull request

### Code Style
- Rust: Use `cargo fmt` and `cargo clippy`
- JavaScript: Follow existing code style
- Commit messages: Use conventional commits format

## Getting Help

If you encounter issues:

1. Check this troubleshooting section
2. Search [GitHub Issues](https://github.com/yourusername/film-grain-generator/issues)
3. Create a new issue with:
   - Your operating system and version
   - Rust and Node.js versions
   - Complete error messages
   - Steps to reproduce

---

**Happy building! 🚀**