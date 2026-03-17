# TKP Language Specification

> **TKP (Toki Pona)** — A minimalist, general-purpose compiled programming language with Toki Pona keywords.
> CLI: `tkp` | File Extension: `.tkp` | Compiler: Rust + LLVM IR → clang

---

## 1. Overview

TKP is a statically-typed compiled language using Toki Pona keywords.
- Toki Pona keywords + Any Unicode identifiers (TP/English/Korean/etc.) are supported.
- C/Rust style syntax.
- Native binary generation via LLVM IR → clang.
- General purpose (system programming to scripting).

---

## 2. Keywords

| Keyword | Toki Pona Meaning | English Equivalent |
|---------|-------------------|-------------------|
| `pali`  | make / do / work  | `fn` / `function` |
| `pana`  | give / send       | `return`          |
| `ijo`   | thing / object    | `let` / `var`     |
| `awen`  | stay / keep       | `const`           |
| `la`    | (context marker)  | `if`              |
| `ante`  | different         | `else`            |
| `sin`   | new / again       | `for`             |
| `lon`   | exist / true      | `while`           |
| `pini`  | end / finish      | `break`           |
| `tawa`  | go / toward       | `continue`        |
| `kin`   | indeed / also     | `true`            |
| `ala`   | no / not / zero   | `false`           |
| `weka`  | away / absent     | `void` / `null`   |
| `toki`  | speak / print     | `print`           |
| `kute`  | listen / hear     | `input`           |
| `kulupu`| group / community | `struct`          |
| `lukin` | look / try        | `try`             |
| `alasa` | hunt / gather     | `catch`           |
| `sama`  | same / similar    | `match`           |
| `jo`    | have / contain    | `import`          |
| `ken`   | can / ability     | `impl`            |
| `nanpa` | number            | `enum`            |
| `insa`  | inside            | `in`              |

---

## 3. Type System

| Type | LLVM Type | Size | Description |
|------|-----------|------|-------------|
| `nanpa_kind` | `i64` | 8 bytes | 64-bit integer |
| `kipisi` | `f64` | 8 bytes | 64-bit float |
| `sitelen` | `i8*` | Pointer | UTF-8 string |
| `lawa` | `i1` | 1 bit | Boolean (kin/ala) |
| `weka` | `void` | — | No return value / null |

---

## 4. Grammar (EBNF)

```ebnf
Program         ::= Declaration*

Declaration     ::= FunctionDecl | VarDecl | ConstDecl | ExprStmt

FunctionDecl    ::= "pali" Identifier "(" ParamList? ")" ("->" Type)? Block
ParamList       ::= Param ("," Param)*
Param           ::= Identifier ":" Type

VarDecl         ::= "ijo" Identifier (":" Type)? "=" Expression
ConstDecl       ::= "awen" Identifier (":" Type)? "=" Expression

Block           ::= "{" Statement* "}"

Statement       ::= VarDecl
                  | ConstDecl
                  | ReturnStmt
                  | IfStmt
                  | ForLoop
                  | WhileLoop
                  | BreakStmt
                  | ContinueStmt
                  | ExprStmt

ReturnStmt      ::= "pana" Expression?
IfStmt          ::= "la" Expression Block ("ante" Block)?
ForLoop         ::= "sin" VarDecl ";" Expression ";" Expression Block
WhileLoop       ::= "lon" Expression Block
BreakStmt       ::= "pini"
ContinueStmt    ::= "tawa"
ExprStmt        ::= Expression

Expression      ::= Assignment
Assignment      ::= Identifier "=" Expression | Logical
Logical         ::= Comparison (("&&" | "||") Comparison)*
Comparison      ::= Additive (("==" | "!=" | "<" | ">" | "<=" | ">=") Additive)*
Additive        ::= Multiplicative (("+" | "-") Multiplicative)*
Multiplicative  ::= Unary (("*" | "/" | "%") Unary)*
Unary           ::= ("-" | "!") Unary | Primary
Primary         ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral
                  | Identifier | FunctionCall | "(" Expression ")"

FunctionCall    ::= Identifier "(" ArgumentList? ")"
ArgumentList    ::= Expression ("," Expression)*

Type            ::= "nanpa_kind" | "kipisi" | "sitelen" | "lawa" | "weka"
Identifier      ::= (Alpha | "_") (AlphaNum | "_")*
```

---

## 5. Examples

### 5.1 Hello World
```tokipona
pali main() -> weka {
    toki("toki a, ma o!")
}
```

### 5.2 Variables and Constants
```tokipona
ijo age = 20
awen name = "TKP"
ijo score: kipisi = 98.5
```

### 5.3 Function Definition and Call
```tokipona
pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    pana a + b
}

pali main() -> weka {
    ijo result = add(3, 5)
    toki(result)
}
```

### 5.4 Logic with `la` (If)
```tokipona
pali absolute(x: nanpa_kind) -> nanpa_kind {
    la x < 0 {
        pana -x
    } ante {
        pana x
    }
}
```

### 5.5 `sin` loop (For)
```tokipona
pali main() -> weka {
    sin ijo i = 0; i < 10; i += 1 {
        toki(i)
    }
}
```

### 5.6 `lon` loop (While)
```tokipona
pali main() -> weka {
    ijo n = 1
    lon n <= 100 {
        toki(n)
        n += 1
    }
}
```

### 5.7 Recursion — Fibonacci
```tokipona
pali fibonacci(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 {
        pana n
    }
    pana fibonacci(n - 1) + fibonacci(n - 2)
}

pali main() -> weka {
    toki(fibonacci(10))
}
```

### 5.8 Factorial
```tokipona
pali factorial(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 {
        pana 1
    }
    pana n * factorial(n - 1)
}

pali main() -> weka {
    toki(factorial(10))
}
```

### 5.9 `pini` / `tawa` (Break / Continue)
```tokipona
pali main() -> weka {
    sin ijo i = 0; i < 20; i += 1 {
        la i % 2 == 0 {
            tawa
        }
        la i > 15 {
            pini
        }
        toki(i)
    }
}
```

### 5.10 String Output
```tokipona
pali greet(name: sitelen) -> weka {
    toki("toki, ")
    toki(name)
    toki("!")
}

pali main() -> weka {
    greet("ma")
    greet("TKP")
}
```

---

## 6. Operators

| Operator | Type | Precedence |
|----------|------|------------|
| `*`, `/`, `%` | Multiplicative | High |
| `+`, `-` | Additive | Medium |
| `==`, `!=`, `<`, `>`, `<=`, `>=` | Comparison | Low |
| `&&` | Logical AND | Lower |
| `||` | Logical OR | Lowest |
| `=`, `+=`, `-=`, `*=`, `/=` | Assignment | Right-associative |

---

## 7. Comments

```tokipona
// Single line comment
// Multi-line comments are not supported in TKP MVP.
```

---

## 8. Compilation Pipeline

```
Source Code (.tkp)
    ↓ Lexer
Token Stream
    ↓ Parser
Abstract Syntax Tree (AST)
    ↓ Code Generator
LLVM IR Text (.ll)
    ↓ clang
Native Binary
```

---

## 9. Limitations (MVP)

- Minimal standard library (only `toki` and `kute`).
- Reference counting based memory management for strings/arrays (cycles leak).
- Single-threaded execution.
- No class support (use `kulupu` instead).
- No standard garbage collector.
- No complex build system (direct invocation of `tkp build`).
