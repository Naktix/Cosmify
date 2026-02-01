#!/bin/bash
set -e

echo "Uninstalling Cosmify"

sudo rm -f /usr/bin/cosmify
sudo rm -f /usr/share/applications/dev.naktix.Cosmify.desktop
sudo rm -f /usr/share/metainfo/dev.naktix.Cosmify.metainfo.xml

echo "Please restart COSMIC or log out/in"
echo ""