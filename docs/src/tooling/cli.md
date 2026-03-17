# CLI Usage

```
tkp interpret <file.tkp>    Run with interpreter
tkp build <file.tkp>        Compile to native binary
tkp run <file.tkp>          Compile and run immediately
tkp repl                    Interactive REPL
tkp lsp                     Start LSP server
```

## Examples

```bash
tkp interpret examples/fibonacci.tkp
tkp build examples/sum.tkp && ./sum
tkp repl
```
