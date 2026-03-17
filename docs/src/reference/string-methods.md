# String Methods

| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.suli_ijo()` | Character count | `"tkptext".suli_ijo()` | `2` |
| `.split(sep)` | Split by separator | `"a,b,c".split(",")` | `["a", "b", "c"]` |
| `.contains(s)` | Contains substring | `"hello".contains("ell")` | `kin` |
| `.replace(from, to)` | Replace | `"hello".replace("l", "r")` | `"herro"` |
| `.trim()` | Trim whitespace | `" hi ".trim()` | `"hi"` |
| `.uppercase()` | Uppercase | `"hello".uppercase()` | `"HELLO"` |
| `.lowercase()` | Lowercase | `"HELLO".lowercase()` | `"hello"` |
| `.starts_with(s)` | Starts with | `"hello".starts_with("he")` | `kin` |
| `.ends_with(s)` | Ends with | `"hello".ends_with("lo")` | `kin` |

## String Indexing

```
ijo s = "tkptext"
toki(s[0])    // tkp
toki(s[1])    // text
```

## String Iteration

```
sin text insa "tkptext" {
    toki(text)
}
```

## Concatenation

```
ijo full = "toki" + "hello"
toki(full)    // toki
```
