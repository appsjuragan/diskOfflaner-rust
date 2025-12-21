#!/bin/bash

# DiskOfflaner Linux Installation Script
# This script installs the application icon and desktop entry

set -e

echo "Installing DiskOfflaner..."

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root (use sudo)"
    exit 1
fi

# Define paths
BINARY_SOURCE="./target/release/diskofflaner"
BINARY_DEST="/usr/local/bin/diskofflaner"
ICON_SOURCE="./assets/icon.png"
ICON_DEST="/usr/share/pixmaps/diskofflaner.png"
DESKTOP_SOURCE="./assets/diskofflaner.desktop"
DESKTOP_DEST="/usr/share/applications/diskofflaner.desktop"

# Check if binary exists
if [ ! -f "$BINARY_SOURCE" ]; then
    echo "Error: Binary not found at $BINARY_SOURCE"
    echo "Please build the application first with: cargo build --release"
    exit 1
fi

# Install binary
echo "Installing binary to $BINARY_DEST..."
cp "$BINARY_SOURCE" "$BINARY_DEST"
chmod +x "$BINARY_DEST"

# Install icon
echo "Installing icon to $ICON_DEST..."
cp "$ICON_SOURCE" "$ICON_DEST"
chmod 644 "$ICON_DEST"

# Install desktop entry
echo "Installing desktop entry to $DESKTOP_DEST..."
cp "$DESKTOP_SOURCE" "$DESKTOP_DEST"
chmod 644 "$DESKTOP_DEST"

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    echo "Updating desktop database..."
    update-desktop-database /usr/share/applications
fi

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    echo "Updating icon cache..."
    gtk-update-icon-cache -f -t /usr/share/pixmaps 2>/dev/null || true
fi

echo ""
echo "✓ Installation complete!"
echo ""
echo "You can now:"
echo "  - Run from terminal: sudo diskofflaner"
echo "  - Launch from application menu: Search for 'DiskOfflaner'"
echo ""
echo "Note: The application requires root privileges to manage disks."
