# Complete API Reference

This page is optimized for LLM consumption. It contains every keyword, type, builtin, method, and operator in TKP in a structured, machine-readable format.

## Language: TKP (tkp)
## File Extension: .tkp
## Execution: `tkp interpret <file>` or `tkp build <file>`

---

## KEYWORDS

```
pali    → function definition
pana    → return
ijo    → mutable variable (let)
awen    → immutable constant (const)
la    → if
ante  → else
sin    → for loop
lon    → while loop
pini    → break
tawa    → continue
insa  → in (for-in iteration)
kulupu    → struct definition
ken    → impl block
nanpa    → enum definition
lukin    → try
alasa    → catch
sama    → match (pattern matching)
jo → import
kin      → true
ala    → false
weka    → null/void
```

## TYPES

```
nanpa_kind    → i64 (64-bit integer)
kipisi    → f64 (64-bit float)
sitelen  → String (UTF-8)
lawa      → bool
weka    → void
[nanpa_kind]  → Array of integers
(nanpa_kind, sitelen) → Tuple
```

## OPERATORS

```
+  -  *  /  %           → arithmetic
== !=  <  >  <=  >=     → comparison
&& ||  !                → logical
=  +=  -=  *=  /=       → assignment
->                      → return type arrow
=>                      → match arm arrow
..                      → range (0..10)
::                      → enum access (direction::up)
.                       → field/method access
```

## BUILTIN FUNCTIONS

```
toki(value...)              → print to stdout
kute()                   → read line from stdin → sitelen
toki_pakala(value...)           → print to stderr
lili_nanpa(x)                → sqrt(x) → kipisi
wawa_nanpa(x)                → abs(x) → number
suli_nanpa(base, exp)        → pow(base, exp) → kipisi
nanpa_ante(x)              → int(x) → nanpa_kind
kipisi_ante(x)              → float(x) → kipisi
suli_ijo(s)                  → len(s) → nanpa_kind
sitelen_pali(template, args...)  → format string → sitelen
lipu_lukin(path)            → read file → sitelen
lipu_sitelen(path, content)      → write file
lipu_sin(path, content)      → append to file
lipu_lon(path)            → file exists → lawa
dictionary(key, value, ...)        → create HashMap → dictionary
kulupu_lukin(sitelen)       → parse JSON → value
kulupu_pali(value)           → value → JSON string
kulupu_pona(value)         → value → pretty JSON string
lipu_lukin_http(url)        → GET request → sitelen
tawa_pana(url, body)    → POST request → sitelen
nasin_alasa(pattern, text)  → find matches → [sitelen]
nasin_sama(pattern, text)  → test match → lawa
nasin_ante(pattern, text, rep) → replace → sitelen
tenpo_ni()               → current datetime → sitelen
suno_ni()               → current date → sitelen
nanpa_tenpo()             → unix timestamp → nanpa_kind
lawa_pali(cmd)             → shell command → sitelen
ma_ijo(name)           → env var → sitelen or weka
toki_ijo()               → CLI args → [sitelen]
lape(ms)           → sleep
type(value)                 → type name → sitelen
```

## MAP METHODS

```
.keys()        → all keys → [T]
.values()        → all values → [T]
.suli_ijo()          → entry count → nanpa_kind
.contains(key)        → key exists → lawa
.remove(key)        → remove entry → removed value
```

## ARRAY METHODS

```
.push(value)        → push element
.remove(index)    → remove at index → removed value
.suli_ijo()          → length → nanpa_kind
.contains(value)        → contains → lawa
.reverse()          → reversed copy → [T]
.sort()          → sorted copy → [T]
.join(sep)  → join to string → sitelen
```

## STRING METHODS

```
.suli_ijo()          → character count → nanpa_kind
.split(sep)    → split → [sitelen]
.contains(part)      → contains → lawa
.replace(before, after)  → replace → sitelen
.trim()  → trim → sitelen
.uppercase()        → uppercase → sitelen
.lowercase()        → lowercase → sitelen
.starts_with(prefix)    → starts with → lawa
.ends_with(suffix)      → ends with → lawa
```

## SYNTAX PATTERNS

```tkp
// Variable declaration
ijo name = value
ijo name: type = value
awen name = value

// Function
pali name(params: type) -> panatype {
    pana value
}

// If/else
la condition {
    ...
} ante la condition2 {
    ...
} ante {
    ...
}

// For loop
sin ijo i = 0; i < n; i += 1 {
    ...
}

// For-in
sin item insa array {
    ...
}

// While
lon condition {
    ...
}

// Struct
kulupu name {
    field: type,
    field2: type
}

// Impl
ken structname {
    pali method(self: structname) {
        ...
    }
}

// Enum
nanpa name {
    variant1,
    variant2
}

// Match
sama value {
    pattern1 => result1
    pattern2 => { ... }
    _ => defaultvalue
}

// Try/catch
lukin {
    ...
} alasa(errorijo) {
    ...
}

// Import
jo "file.tkp"

// Closure
ijo f = pali(x: nanpa_kind) { pana x * 2 }

// Tuple
ijo t = (1, "hello", kin)
t.0  // 1

// Range
0..10  // [0, 1, 2, ..., 9]

// Array
ijo arr = [1, 2, 3]
arr[0]      // indexing
arr[-1]     // negative indexing
arr[0] = 99 // mutation
```
