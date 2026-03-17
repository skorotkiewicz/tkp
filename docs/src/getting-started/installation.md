# Installation

## Prerequisites

- [Rust](https://rustup.rs) 1.70+
- clang (for `tkp build` / `tkp run`) — optional for interpreter-only use

### macOS
```bash
xcode-select --install
```

### Linux
```bash
sudo apt install clang
```

## Install TKP

```bash
git clone https://github.com/skorotkiewicz/tkp.git
cd tkp
cargo install --path .
```

`tkp` is now available globally.

## Verify Installation

```bash
tkp --help
```
