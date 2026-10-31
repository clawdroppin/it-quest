#!/usr/bin/env bash
# Build the portable Linux AppImage (and a .deb) on a Linux machine.
# Usage: ./scripts/build-linux.sh   →   dist-linux/IT Quest_<version>_amd64.AppImage
set -euo pipefail
cd "$(dirname "$0")/.."

if command -v apt-get >/dev/null; then
  echo "Installing Tauri's system libraries (needs sudo)..."
  sudo apt-get update
  sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf build-essential curl file
fi

command -v cargo >/dev/null || { echo "Install Rust first: https://rustup.rs"; exit 1; }
command -v npm >/dev/null || { echo "Install Node.js 20 or newer first: https://nodejs.org"; exit 1; }

npm ci
npx tauri build

mkdir -p dist-linux
cp src-tauri/target/release/bundle/appimage/*.AppImage dist-linux/
cp src-tauri/target/release/bundle/deb/*.deb dist-linux/
chmod +x dist-linux/*.AppImage
echo
echo "Done. Copy the .AppImage anywhere and run it; your progress is saved next to it."
ls -l dist-linux
