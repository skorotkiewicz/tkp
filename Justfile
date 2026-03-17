# TKP (Toki Pona) Justfile
# A command runner for development tasks

# Default to listing recipes
default:
	@just --list

# --- Core Development ---

# Build the project
build:
	cargo build

# Build with optimizations
release:
	cargo build --release

# Run the project with arguments
run *ARGS:
	cargo run -- {{ARGS}}

# Run a .tkp file with the interpreter
interpret FILE:
	cargo run -- interpret {{FILE}}

# Run a .tkp file with the JIT/Compiler
compile-run FILE:
	cargo run -- run {{FILE}}

# Start the REPL
repl:
	cargo run -- repl

# Start the LSP server
lsp:
	cargo run -- lsp

# Run all tests
test:
	cargo test

# Check code for warnings and format
check:
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt -- --check

# Format all files
fmt:
	cargo fmt

# Clean build artifacts
clean:
	cargo clean

# --- Documentation ---

# Build documentation with mdbook
docs-build:
	cd docs && mdbook build

# Serve documentation locally
docs-serve:
	cd docs && mdbook serve

# --- Web Playground ---

# Build WASM for the web playground
web-build:
	wasm-pack build --target web --out-name tkp --out-dir web --release

# Serve the web playground locally
web-serve:
	cd web && python3 -m http.server 8000

# --- VS Code Extension ---

# Install extension dependencies
vscode-install:
	cd editors/vscode && npm install

# Build the VS Code extension
vscode-build:
	cd editors/vscode && npm run compile

# Package the extension
vscode-pkg:
	cd editors/vscode && npx vsce package

# --- Convenience ---

# Install the binary to ~/.cargo/bin
install:
	cargo install --path .

# Run clippy fix to automatically resolve issues
clippy-fix:
	cargo clippy --fix --allow-dirty --allow-staged
