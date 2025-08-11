#!/bin/bash

# Build script for Fenex WASM demo

echo "🏗️  Building Fenex WASM module..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build WASM module
echo "🔧 Compiling Rust to WASM..."
wasm-pack build --target web --out-dir docs/pkg --no-typescript

if [ $? -eq 0 ]; then
    echo "✅ WASM build successful!"
    echo ""
    echo "🚀 To run the demo:"
    echo "1. Start a local server (required for WASM loading):"
    echo "   python -m http.server 8000"
    echo "   # or use any other static file server"
    echo ""
    echo "2. Open your browser to:"
    echo "   http://localhost:8000/docs"
    echo ""
    echo "📦 To deploy to GitHub Pages:"
    echo "1. Commit and push all files including docs/pkg/"
    echo "2. Go to your repository Settings > Pages"
    echo "3. Set source to 'Deploy from a branch'"
    echo "4. Select 'main' branch and '/docs' folder"
    echo "5. Your site will be available at: https://USERNAME.github.io/REPOSITORY/"
else
    echo "❌ WASM build failed!"
    exit 1
fi
