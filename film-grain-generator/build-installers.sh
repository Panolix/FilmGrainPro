#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🎬 Film Grain Generator - Building Installers"
echo "============================================="
echo ""

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle
npm run clean 2>/dev/null || true

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build for current platform
echo "🔨 Building installer for current platform..."
npm run tauri build

# Replace the DMG volume icon with a rounded-corner version so the mounted
# installer icon matches the app icon instead of showing hard square edges
if [ "$(uname -s)" = "Darwin" ]; then
    echo "🖼️  Fixing DMG volume icons..."
    shopt -s nullglob
    DMGS=(src-tauri/target/release/bundle/dmg/*.dmg src-tauri/target/*/release/bundle/dmg/*.dmg)
    shopt -u nullglob
    for dmg in "${DMGS[@]}"; do
        bash "$SCRIPT_DIR/scripts/fix-dmg-volume-icon.sh" "$dmg"
    done
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "📁 Installers created in:"
echo "   src-tauri/target/release/bundle/"
echo ""

# Show what was built
if [ -d "src-tauri/target/release/bundle" ]; then
    echo "🎉 Generated files:"
    find src-tauri/target/release/bundle -name "*.dmg" -o -name "*.exe" -o -name "*.msi" -o -name "*.deb" -o -name "*.AppImage" | while read file; do
        size=$(du -h "$file" | cut -f1)
        echo "   📦 $(basename "$file") ($size)"
    done
fi

