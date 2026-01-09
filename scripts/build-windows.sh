#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Define the backend path relative to the script
BACKEND_DIR="$SCRIPT_DIR/../backend"

echo "-------------------------------------------------------"
echo "🚀 Starting Cross-Compilation: WSL -> Windows (.exe)"
echo "Project Path: $BACKEND_DIR"
echo "-------------------------------------------------------"

# 1. Ensure we are in the backend directory
if [ -d "$BACKEND_DIR" ]; then
    cd "$BACKEND_DIR"
else
    echo "❌ Error: Backend directory not found at $BACKEND_DIR"
    exit 1
fi

# 2. Ensure the Windows target is installed in Rust
echo "Checking Rust target..."
rustup target add x86_64-pc-windows-gnu

# 3. Build the project
# --release: Optimizes for performance (essential for 60Hz telemetry)
echo "Compiling binaries in /backend... this may take a minute."
cargo build --release --target x86_64-pc-windows-gnu

echo "-------------------------------------------------------"
echo "✅ Build Successful!"
echo "📍 Location: backend/target/x86_64-pc-windows-gnu/release/iracing-telemetry.exe"
echo "-------------------------------------------------------"
echo "💡 Tip: You can now run this .exe from your Windows PowerShell."