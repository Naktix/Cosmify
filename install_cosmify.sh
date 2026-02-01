#!/bin/bash
set -e

echo "Installing Cosmify"

cargo build --release
sudo install -Dm755 target/release/cosmify /usr/bin/cosmify
sudo install -Dm644 dev.naktix.Cosmify.desktop /usr/share/applications/dev.naktix.Cosmify.desktop
sudo install -Dm644 dev.naktix.Cosmify.metainfo.xml /usr/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo "Please restart COSMIC or log out/in"
echo ""