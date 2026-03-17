# Learn TKP (Toki Pona) Programming Language

TKP is a minimalist, statically-typed, compiled programming language where every keyword is a **Toki Pona** word. It leverages LLVM for high performance while keeping the syntax simple and fun.

---

## 1. Quick Start: Hello World

Save this as `hello.tkp`:

```tokipona
pali main() {
    toki("toki a, ma o!")
}
```

Run it with the interpreter:
```bash
tkp interpret hello.tkp
```

Or build a native binary:
```bash
tkp build hello.tkp -o hello
./hello
```

---

## 2. Keywords Cheat Sheet

| Keyword | English Equivalent | Toki Pona Literal |
| :--- | :--- | :--- |
| `pali` | `fn` / `function` | make / do / work |
| `pana` | `return` | give / send |
| `ijo` | `let` / `var` | thing / object |
| `awen` | `const` | stay / keep |
| `la` | `if` | (context marker) |
| `ante` | `else` | different |
| `sin` | `for` | new / again |
| `insa` | `in` (for-in loop) | inside |
| `lon` | `while` | exist / true |
| `pini` | `break` | end / finish |
| `tawa` | `continue` | go / toward |
| `kin` | `true` | indeed / also |
| `ala` | `false` | no / not / zero |
| `weka` | `void` / `null` | away / absent |
| `toki` | `print` | speak / print |
| `kute` | `input` | listen / hear |
| `kulupu` | `struct` | group / community |
| `nanpa` | `enum` | number |
| `lukin` | `try` | look / try |
| `alasa` | `catch` | hunt / gather |
| `sama` | `match` | same / similar |
| `jo` | `import` | have / contain |
| `ken` | `impl` | can / ability |

---

## 3. Variables & Types

### Declarations
Use `ijo` for variables and `awen` for constants.
```tokipona
ijo age = 20           // Variable
awen name = "TKP"      // Constant
ijo score: kipisi = 9.5 // With explicit type
```

### Core Types
- `nanpa_kind`: 64-bit integer
- `kipisi`: 64-bit float
- `sitelen`: String
- `lawa`: Boolean (`kin` or `ala`)
- `weka`: Void / Null

---

## 4. Functions

### Definitions
Functions start with `pali`.
```tokipona
pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    pana a + b
}
```

### Lambdas (Closures)
```tokipona
ijo square = pali(x: nanpa_kind) { pana x * x }
toki(square(5)) // 25
```

---

## 5. Control Flow

### If-Else (`la` / `ante`)
```tokipona
la x > 10 {
    toki("suli!")
} ante la x < 0 {
    toki("lili!")
} ante {
    toki("pona")
}
```

### Loops (`sin` / `lon`)
**For loop:**
```tokipona
sin ijo i = 0; i < 10; i += 1 {
    toki(i)
}
```

**For-in loop (Array/String):**
```tokipona
ijo names = ["jan", "pona", "ma"]
sin name insa names {
    toki(name)
}
```

**While loop:**
```tokipona
ijo n = 0
lon n < 5 {
    toki(n)
    n += 1
}
```

---

## 6. Data Structures

### Arrays & Tuples
```tokipona
ijo list = [1, 2, 3]    // Array
ijo pair = (10, "ten") // Tuple
toki(list[0])          // Index access
toki(pair.1)           // Tuple index access
```

### Structs (`kulupu`)
```tokipona
kulupu Jan {
    name: sitelen,
    age: nanpa_kind
}

// Instantiate
ijo me = Jan { name: "Antigravity", age: 1 }
toki(me.name)
```

### Enums (`nanpa`)
```tokipona
nanpa Color {
    Red,
    Green,
    Blue
}

ijo my_color = Color::Red
```

---

## 7. Advanced Features

### Pattern Matching (`sama`)
```tokipona
sama x {
    1 => { toki("one") },
    2 => { toki("two") },
    _ => { toki("mute") }
}
```

### Error Handling (`lukin` / `alasa`)
```tokipona
lukin {
    // risky code
} alasa(err) {
    toki("pakala: " + err)
}
```

### Impls (`ken`)
```tokipona
ken Jan {
    pali greet() {
        toki("toki, mi jan " + self.name)
    }
}
me.greet()
```

---

## 8. Standard Library Highlights

- `toki(...)`: Print to console.
- `kute()`: Read string from stdin.
- `sitelen_pali(template, ...)`: Format strings (`"toki, {0}!"`).
- `nasin(val)`: Get type name of a value.
- `lipu_lukin(path)`: Read file to string.
- `lape(ms)`: Sleep.
- `lawa_pali(cmd)`: Run shell command.

---

## 9. Project Structure
If you want to split your code, use `jo`:
```tokipona
jo "math.tkp"
toki(math::add(1, 2))
```

---

*Enjoy coding in Toki Pona! o pali pona!*
