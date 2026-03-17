# TKP Language for VS Code

This extension provides syntax highlighting and LSP support for the TKP (Toki Pona) programming language.

## Features

- **Syntax Highlighting**: Supports keywords, types, strings, comments, etc.
- **LSP Integration**: Connects to the `tkp lsp` server for hover documentation and completion.

## Configuration

You can configure the server path in your VS Code settings:

```json
{
  "tkp.serverPath": "tkp"
}
```

The default is `tkp`, assuming it's in your PATH.

## Development

To build the extension:

1.  Run `npm install`
2.  Run `npm run compile`
3.  Launch the extension (F5)
