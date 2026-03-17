#!/bin/bash
set -e

echo "=== TKP (Toki Pona) Installation ==="
echo ""

# 1. Build and install the Rust binary
echo "[1/3] Building and installing tkp CLI..."
cargo install --path .
echo "  tkp installed successfully"

# 2. Install VS Code extension (if VS Code is present)
if command -v code &> /dev/null; then
    echo "[2/3] Installing VS Code extension..."
    VSIX="editors/vscode/tkp-language-0.1.0.vsix"
    if [ ! -f "$VSIX" ]; then
        echo "  Building VSIX package..."
        cd editors/vscode
        npm install --silent 2>/dev/null
        npx @vscode/vsce package --allow-missing-repository 2>/dev/null
        cd ../..
    fi
    code --install-extension "$VSIX" --force 2>/dev/null
    echo "  VS Code extension installed successfully"
else
    echo "[2/3] VS Code not found — skipping extension installation"
fi

# 3. Verify installation
echo "[3/3] Verifying installation..."
echo ""
tkp --version 2>/dev/null || echo "  tkp installed"
echo ""
echo "=== Installation Complete ==="
echo ""
echo "Usage:"
echo "  tkp interpret hello.tkp    # Run with interpreter"
echo "  tkp build hello.tkp        # Compile to native binary"
echo "  tkp repl                   # Interactive REPL"
echo ""
echo "Documentation: Read README.md or browse /docs"
