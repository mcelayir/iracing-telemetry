#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "-------------------------------------------------------"
echo "🚀 Starting Cross-Compilation: WSL -> Windows (.exe)"
echo "-------------------------------------------------------"

# 1. Ensure the Windows target is installed in Rust
echo "Checking Rust target..."
rustup target add x86_64-pc-windows-gnu

# 2. Build the project
# --release: Optimizes the code for performance (essential for 60Hz telemetry)
# --target: Specifies we want a Windows executable
echo "Compiling binaries... this may take a minute on first run."
cargo build --release --target x86_64-pc-windows-gnu

echo "-------------------------------------------------------"
echo "✅ Build Successful!"
echo "📍 Location: target/x86_64-pc-windows-gnu/release/iracing-telemetry.exe"
echo "-------------------------------------------------------"
echo "💡 Tip: You can now run this .exe from your Windows PowerShell."