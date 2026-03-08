#!/bin/bash
set -e

echo "Uninstalling Cosmify..."

sudo rm -f /usr/bin/cosmify
sudo rm -f /usr/share/applications/dev.naktix.Cosmify.desktop
sudo rm -f /usr/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo "Cleaning up..."
cargo clean

echo "Building Cosmify..."
cargo build --release

echo "Installing Cosmify..."
sudo install -Dm755 target/release/cosmify /usr/bin/cosmify
sudo install -Dm644 dev.naktix.Cosmify.desktop /usr/share/applications/dev.naktix.Cosmify.desktop
sudo install -Dm644 dev.naktix.Cosmify.metainfo.xml /usr/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo ""
echo "Please restart COSMIC or log out/in"
echo ""
