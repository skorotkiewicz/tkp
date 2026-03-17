# Architecture

## Compiler Pipeline

```
Source (.tkp)
    │
    ▼
┌─────────────┐
│   Lexer     │  Source text → Token stream
│ (lexer.rs)  │  "pali" → Token::pali
└──────┬──────┘
       ▼
┌─────────────┐
│   Parser    │  Token stream → AST
│ (parser.rs) │  Recursive descent, precedence climbing
└──────┬──────┘
       ▼
┌─────────────┐
│    AST      │  Tree representation of the program
│  (ast.rs)   │  Expr, StmtKind, Pattern, Type
└──────┬──────┘
       │
       ├─────────────────┐
       ▼                 ▼
┌─────────────┐   ┌─────────────┐
│ Interpreter │   │  CodeGen    │
│(interpreter)│   │(codegen.rs) │
│             │   │             │
│ Tree-walking│   │ LLVM IR text│
│ execution   │   │ generation  │
└─────────────┘   └──────┬──────┘
                         ▼
                    clang → Binary
```

## Source Files

| File | Lines | Purpose |
|------|-------|---------|
| `lexer.rs` | ~550 | Tokenization, toki pona keyword recognition |
| `parser.rs` | ~1280 | Recursive descent parser, precedence climbing |
| `ast.rs` | ~270 | AST node definitions (Expr, Stmt, Type, Pattern) |
| `interpreter.rs` | ~1760 | Tree-walking interpreter, builtins, methods |
| `codegen.rs` | ~1430 | LLVM IR text generation (toki pona→ASCII sanitization) |
| `typechecker.rs` | ~280 | Compile-time type checker (warning mode) |
| `lsp.rs` | ~330 | LSP server (hover, completion) |
| `main.rs` | ~310 | CLI entry point (clap) |
| `builtins/` | — | Builtin function catalog (math, io, string, system) |

## Builtin Module Structure

```
src/builtins/
├── mod.rs      — module declarations
├── math.rs     — lili_nanpa, wawa_nanpa, suli_nanpa, nanpa_ante, kipisi_ante
├── io.rs       — toki, kute, sitelen_pali, lipu_lukin/write
├── string.rs   — suli_ijo, split, jo, replace, upper, lower
└── system.rs   — lawa_pali, HTTP, nasin, json, datetime
```

## Codegen: toki pona Identifier TKPdling

LLVM IR only allows ASCII identifiers. TKP's codegen sanitizes toki pona variable and function names using Unicode hex encoding:

```
ijo double = ...  →  %var_uB450uBC30 = alloca i64
pali greet() { }  →  define void @uc778uc0ac() { }
```

This allows toki pona-named functions and variables to compile to native binaries.
