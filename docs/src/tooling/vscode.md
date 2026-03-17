# VS Code Extension

## Install from Source

```bash
cd editors/vscode
npm install
npm run compile
```

Press `F5` in VS Code to launch with the extension loaded.

## Features

- Syntax highlighting for `.tkp` files
- Keyword, type, builtin, string, number, comment coloring
- Auto-closing brackets and quotes
- LSP integration (hover docs + completion)

## LSP Setup

Make sure `tkp` is in your PATH:

```bash
cargo install --path .
```

The extension automatically starts `tkp lsp` when you open a `.tkp` file.
