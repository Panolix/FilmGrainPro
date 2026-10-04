#!/bin/bash
set -euo pipefail

DMG="${1:-}"
if [ -z "$DMG" ] || [ ! -f "$DMG" ]; then
  echo "Usage: $(basename "$0") <path/to/installer.dmg>"
  echo ""
  echo "Replaces the DMG volume icon with a rounded-corner version so the"
  echo "mounted installer icon matches the app icon rendering on macOS 26+."
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VOLICON="$SCRIPT_DIR/../src-tauri/icons/icon_volume.icns"

if [ ! -f "$VOLICON" ]; then
  echo "Volume icon not found: $VOLICON"
  exit 1
fi

if codesign -dv "$DMG" >/dev/null 2>&1; then
  echo "Warning: $DMG is code-signed; replacing the volume icon will invalidate the signature."
fi

WORK_DIR="$(mktemp -d)"
MOUNT_DIR="$WORK_DIR/mnt"
RW_DMG="$WORK_DIR/rw.dmg"
FIXED_DMG="$WORK_DIR/fixed.dmg"
mkdir -p "$MOUNT_DIR"

cleanup() {
  hdiutil detach "$MOUNT_DIR" >/dev/null 2>&1 || true
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

echo "Converting to read-write image..."
hdiutil convert "$DMG" -format UDRW -ov -o "$RW_DMG" >/dev/null

echo "Mounting..."
hdiutil attach "$RW_DMG" -readwrite -noverify -noautoopen -nobrowse -mountpoint "$MOUNT_DIR" >/dev/null

echo "Installing rounded volume icon..."
cp "$VOLICON" "$MOUNT_DIR/.VolumeIcon.icns"
SetFile -c icnC "$MOUNT_DIR/.VolumeIcon.icns"
SetFile -a C "$MOUNT_DIR"

hdiutil detach "$MOUNT_DIR" >/dev/null

echo "Compressing..."
hdiutil convert "$RW_DMG" -format UDZO -imagekey zlib-level=9 -o "$FIXED_DMG" >/dev/null

mv "$FIXED_DMG" "$DMG"
echo "Fixed volume icon: $DMG"
