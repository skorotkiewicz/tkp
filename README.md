# TKP  
A compiled programming language with Toki Pona keywords written in Rust

## Philosophy

- Extreme simplicity  
- Toki Pona semantics & vocabulary  
- Native speed via LLVM

## Core Features

- Keywords: `pali`, `pana`, `ijo`, `la`, `sin`, `sama`, `lukin alasa`, …
- Static typing: `nanpa_kind`, `kipisi`, `sitelen`, `lawa`
- Structures (`kulupu`), closures, pattern matching
- Try–catch: `lukin {…} alasa(e) {…}`
- Interpreter, JIT, native binaries, REPL, LSP

### Hello

```tokipona
toki("toki a! ni li pona.")
```

```bash
tkp interpret hello.tkp
tkp build hello.tkp && ./hello
```

### Fibonacci
```tokipona
pali fibonacci(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 { pana n }
    pana fibonacci(n-1) + fibonacci(n-2)
}

toki(fibonacci(10))   # 55
```

## Keywords (selection)
|Toki Pona|Meaning|Use|
|---|---|---|
pali|do/work|function
pana|give|return
ijo|thing|let/var
la|context|if
ante|different|else
sin|new/again|for
lon|exist/true|while
sama|same|match
toki|speak|print
lukin|look/try|try
alasa|hunt/catch|catch
kulupu|group|struct

## Installation
```bash
git clone https://github.com/skorotkiewicz/tkp.git
cd tkp
cargo build --release
# copy target/release/tkp to PATH
```

---
## License
MIT

---

<p align="center">
<em>TKP — o pali e pona! (Go do good!)</em>
</p>