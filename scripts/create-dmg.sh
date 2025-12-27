#!/bin/bash

# create-dmg.sh - Create a beautiful DMG for cc-session-manager
# Usage: ./create-dmg.sh [--sign]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
TAURI_DIR="$ROOT_DIR/src-tauri"
BUNDLE_DIR="$ROOT_DIR/target/release/bundle"
DMG_NAME="cc-session-manager"
SIGN=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --sign)
            SIGN=true
            shift
            ;;
    esac
done

echo "=== Creating DMG for cc-session-manager ==="
echo "Root dir: $ROOT_DIR"
echo "Bundle dir: $BUNDLE_DIR"

# Clean up previous builds
rm -rf "$BUNDLE_DIR/dmg" 2>/dev/null || true
rm -f "$DMG_NAME.dmg" 2>/dev/null || true
rm -rf "/tmp/cc-session-manager-dmg" 2>/dev/null || true

# Build the app (skip if already built)
APP_BUNDLE=$(find "$BUNDLE_DIR" -name "*.app" -type d 2>/dev/null | head -1)
if [ -z "$APP_BUNDLE" ]; then
    echo "Building Tauri app..."
    cd "$TAURI_DIR"
    cargo tauri build --bundles dmg 2>/dev/null || cargo tauri build 2>/dev/null || {
        echo "Note: Falling back to universal build..."
        cd "$TAURI_DIR"
        cargo tauri build
    }
fi

# Find the .app bundle
APP_BUNDLE=$(find "$BUNDLE_DIR" -name "*.app" -type d | head -1)
if [ -z "$APP_BUNDLE" ]; then
    echo "Error: Could not find .app bundle"
    ls -la "$BUNDLE_DIR" 2>/dev/null || echo "Bundle directory not found"
    exit 1
fi

echo "Found app bundle: $APP_BUNDLE"

# Create a temporary directory for the DMG contents
DMG_TEMP="/tmp/cc-session-manager-dmg"
mkdir -p "$DMG_TEMP"

# Copy the app bundle
cp -R "$APP_BUNDLE" "$DMG_TEMP/"

# Create the Applications symlink
ln -sf "/Applications" "$DMG_TEMP/Applications"

# Create the DMG using create-dmg if available, otherwise use hdiutil
DMG_OUTPUT="$ROOT_DIR/$DMG_NAME.dmg"

if command -v create-dmg &> /dev/null; then
    echo "Using create-dmg..."
    create-dmg \
        --volname "$DMG_NAME" \
        --volicon "$TAURI_DIR/icons/icon.icns" \
        --background "$SCRIPT_DIR/assets/dmg_background.png" \
        --window-pos 200 200 \
        --window-size 660 400 \
        --icon-size 100 \
        --icon "$DMG_NAME.app" 150 180 \
        --app-drop-link 510 180 \
        "$DMG_OUTPUT" \
        "$DMG_TEMP"

    if [ $? -eq 0 ]; then
        echo "✓ DMG created successfully: $DMG_OUTPUT"
    else
        echo "create-dmg failed, falling back to hdiutil..."
        hdiutil create -format UDBZ -srcfolder "$DMG_TEMP" -volname "$DMG_NAME" "$DMG_OUTPUT"
        echo "✓ DMG created with hdiutil: $DMG_OUTPUT"
    fi
else
    echo "Using hdiutil..."
    hdiutil create -format UDBZ -srcfolder "$DMG_TEMP" -volname "$DMG_NAME" -ov -fs "HFS+" -srcfolder "$DMG_TEMP" "$DMG_OUTPUT"
    echo "✓ DMG created: $DMG_OUTPUT"
fi

# Sign the DMG if requested
if [ "$SIGN" = true ]; then
    if command -v codesign &> /dev/null; then
        echo "Signing DMG..."
        codesign --sign "-" --deep --force --timestamp "$DMG_OUTPUT" 2>/dev/null || {
            echo "Note: Signing failed (may need valid certificate)"
        }
    fi
fi

# Cleanup
rm -rf "$DMG_TEMP"

echo ""
echo "=== Done! ==="
echo "DMG file: $DMG_OUTPUT"
echo ""
echo "To create a signed DMG, run: ./scripts/create-dmg.sh --sign"
