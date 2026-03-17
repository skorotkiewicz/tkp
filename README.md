# TKP (Toki Pona) Programming Language

> A minimalist, general-purpose compiled language with Toki Pona keywords — written in Rust

<p align="center">
  <img src="assets/banner.png" alt="TKP — Programming Simplicity Banner" width="600">
</p>

---

## Mission

TKP is not just a programming language. It is a linguistic experiment with three goals:

**1. Simplicity as a First-Class Feature**
Inspired by [Toki Pona](https://tokipona.org/), a philosophical artistic constructed language with only ~120 words, TKP asks: How much complexity can we strip away from a programming language while remaining powerful? 

**2. A Language for Humans**
Most programming languages are built on English logic. TKP uses the logic of "goodness" (pona) and "simplicity" (lili). Every keyword is a real Toki Pona word, chosen for its philosophical meaning.

**3. Native Performance, Minimal Footprint**
While the keywords are simple, the backend is not. TKP compiles to native binaries via LLVM IR, providing performance that matches traditional compiled languages.

---

## About

TKP is a statically-typed, compiled programming language where every keyword is written in Toki Pona. It compiles to native binaries through LLVM IR and includes a tree-walking interpreter for instant execution. The toolchain is written entirely in Rust.

---

## Features

- **Toki Pona Keywords** — `pali`, `la`, `sin`, `ijo` — write logic in the world's simplest language.
- **Minimalist Identifiers** — name your variables and functions in any script, but Toki Pona is preferred!
- **Compiled & JIT** — generates LLVM IR → clang → native binary, or run instantly with the JIT backend.
- **Interpreter Mode** — run scripts instantly without any dependencies.
- **Interactive REPL** — start the `tp>` shell with `tkp repl`.
- **LSP Server** — `tkp lsp` for editor hover docs and completion.
- **Static Typing** — 4 primary types: `nanpa_kind` (int), `kipisi` (float), `sitelen` (string), `lawa` (bool).
- **Structures & Methods** — `kulupu` and `ken` for object-oriented-like patterns.
- **Closures** — function capture like in modern functional languages.
- **Pattern Matching** — `sama` pattern matching for elegant branching.
- **Error Handling** — `lukin { ... } alasa(error) { ... }` (try/catch).

---

## Quick Start

Create `hello.tkp`:

```tokipona
// hello.tkp
toki("toki a! ni li toki pona.")
```

Run it immediately:

```bash
tkp interpret hello.tkp
# Output: toki a! ni li toki pona.
```

Compile to a native binary:

```bash
tkp build hello.tkp
./output
```

---

## Practical Examples

### Fibonacci Sequence

```tokipona
pali fibonacci(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 {
        pana n
    }
    pana fibonacci(n - 1) + fibonacci(n - 2)
}

toki(fibonacci(10)) // 55
```

### Word Counter

```tokipona
ijo text = "toki pona li toki pona"
ijo words = text.split(" ")
ijo counts = []

sin ijo i = 0; i < words.suli_ijo(); i += 1 {
    toki(words[i])
}
```

---

## Installation

### Prerequisites

- [Rust](https://rustup.rs) (1.70+)
- [Clang/LLVM](https://llvm.org) (required for `build` and `run` commands)

### Install

```bash
git clone https://github.com/your-username/tkp.git
cd tkp
cargo build --release
sudo cp target/release/tkp /usr/local/bin/
```

### Command Runner (just)

We use `just` for common development tasks. Install it via `cargo install just`.

```bash
just build          # Build the project
just test           # Run all tests
just check          # Run clippy and format check
just run <args>     # Run the project
just web-build      # Build the web playground (wasm)
just docs-serve     # Preview documentation
```

---

## CLI Usage

```bash
tkp interpret <file.tkp>    # Run with interpreter (no dependencies)
tkp run <file.tkp>          # Compile & execute instantly using JIT/Clang
tkp build <file.tkp>        # Compile to native binary
tkp repl                    # Start the interactive shell
tkp lsp                     # Start the LSP server
```

---

## Keyword Reference (Toki Pona)

| Keyword | Toki Pona Meaning | Programming Meaning |
|---------|-------------------|---------------------|
| `pali`  | make / do / work  | `fn` / `function`   |
| `pana`  | give / send       | `return`            |
| `ijo`   | thing / object    | `let` / `var`       |
| `awen`  | stay / keep       | `const`             |
| `la`    | (context marker)  | `if`                |
| `ante`  | different         | `else`              |
| `sin`   | new / again       | `for`               |
| `lon`   | exist / true      | `while`             |
| `pini`  | end / finish      | `break`             |
| `tawa`  | go / toward       | `continue`          |
| `kin`   | indeed / also     | `true`              |
| `ala`   | no / not / zero   | `false`             |
| `weka`  | away / absent     | `void` / `null`     |
| `toki`  | speak / print     | `print`             |
| `kute`  | listen / hear     | `input`             |
| `kulupu`| group / community | `struct`            |
| `lukin` | look / try        | `try`               |
| `alasa` | hunt / gather     | `catch`             |
| `sama`  | same / similar    | `match`             |

---

## Design and Architecture

TKP follows a classic compiler pipeline:

1. **Lexer** (`lexer.rs`): Converts Toki Pona source into tokens.
2. **Parser** (`parser.rs`): Recursive descent parser building the AST.
3. **Type Checker** (`typechecker.rs`): Validates types before execution.
4. **Interpreter** (`interpreter.rs`): Tree-walking execution for fast scripts.
5. **CodeGen** (`codegen.rs`): Generates human-readable LLVM IR.

---

## License

MIT

---

<p align="center">
  <em>TKP — o pali e pona! (Go do good!)</em>
</p>
