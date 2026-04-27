#!/bin/bash
set -e

read -p "Username: " TARGET_USER

if [ -z "$TARGET_USER" ]; then
    echo "error: a username must be given"
    exit 1
fi

TARGET_HOME="/home/$TARGET_USER"

echo "Uninstalling Cosmify for $TARGET_USER..."

rm -f "$TARGET_HOME"/.local/bin/cosmify
rm -f "$TARGET_HOME"/.local/share/applications/dev.naktix.Cosmify.desktop
rm -f "$TARGET_HOME"/.local/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo "Cosmify was deleted for $TARGET_USER!"
echo "Please restart COSMIC or log out/in"
echo ""
