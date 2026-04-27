#!/bin/bash
set -e

read -p "Username: " TARGET_USER

if [ -z "$TARGET_USER" ]; then
    echo "error: a username must be given"
    exit 1
fi

TARGET_HOME="/home/$TARGET_USER"
BIN_DEST="$TARGET_HOME/.local/bin/cosmify"

echo "Building Cosmify..."
cargo build --release

echo "Installing files at home of $TARGET_USER..."

mkdir -p "$TARGET_HOME"/.local/bin
mkdir -p "$TARGET_HOME"/.local/share/applications
mkdir -p "$TARGET_HOME"/.local/share/metainfo

install -Dm755 target/release/cosmify "$BIN_DEST"
install -Dm644 dev.naktix.Cosmify.desktop "$TARGET_HOME"/.local/share/applications/dev.naktix.Cosmify.desktop
install -Dm644 dev.naktix.Cosmify.metainfo.xml "$TARGET_HOME"/.local/share/metainfo/dev.naktix.Cosmify.metainfo.xml

sed -i "s|^Exec=.*|Exec=$BIN_DEST|" "$TARGET_HOME"/.local/share/applications/dev.naktix.Cosmify.desktop

chown "$TARGET_USER":"$TARGET_USER" "$BIN_DEST"
chown "$TARGET_USER":"$TARGET_USER" "$TARGET_HOME"/.local/share/applications/dev.naktix.Cosmify.desktop
chown "$TARGET_USER":"$TARGET_USER" "$TARGET_HOME"/.local/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo "Installation for $TARGET_USER was completed!"
echo "Please restart COSMIC or log out/in"
echo ""
