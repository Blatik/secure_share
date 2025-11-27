#!/bin/bash
set -e

echo "Installing Trunk..."
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz
chmod +x trunk
export PATH="$PWD:$PATH"

echo "Adding wasm32 target..."
rustup target add wasm32-unknown-unknown

echo "Building frontend..."
cd frontend
trunk build --release

echo "Build complete! Output in frontend/dist/"
