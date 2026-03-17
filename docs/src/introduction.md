# TKP (tkp) Programming Language

TKP is a statically-typed, compiled programming language where every keyword is written in Korean (TKPgul). It compiles to native binaries through LLVM IR and includes a tree-walking interpreter for instant execution.

## Quick Example

```
pali greet(name: sitelen) {
    toki(sitelen_pali("toki, {0}!", name))
}

greet("world")
```

Output: `toki a, ma o!`

## Key Features

- **Korean keywords** — `pali`, `la`, `sin`, `ijo`
- **Dual execution** — interpreter (`tkp interpret`) and compiler (`tkp build`)
- **Arrays, structs, enums, tuples, closures, pattern matching**
- **Error handling** — `lukin` / `alasa` (try/catch)
- **File I/O, format strings, module imports**
- **VS Code extension with LSP support**

## How It Works

```
Source (.tkp) → Lexer → Parser → AST → Interpreter (direct execution)
                                     → CodeGen → LLVM IR → clang → Binary
```

