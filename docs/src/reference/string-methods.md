# String Methods

| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| .suli_ijo() | Character count | `"tkptext".suli_ijo()` | `2` |
| .tu(sep) | Split by separator | `"a,b,c".tu(",")` | `["a", "b", "c"]` |
| .jo(s) | Contains substring | `"hello".jo("ell")` | `kin` |
| .ante_ijo(from, to) | Replace | `"hello".ante_ijo("l", "r")` | `"herro"` |
| .pona_ijo() | Trim whitespace | `" hi ".pona_ijo()` | `"hi"` |
| .suli_sitelen() | Uppercase | `"hello".suli_sitelen()` | `"HELLO"` |
| .lili_sitelen() | Lowercase | `"HELLO".lili_sitelen()` | `"hello"` |
| .open_sitelen(s) | Starts with | `"hello".open_sitelen("he")` | `kin` |
| .pini_sitelen(s) | Ends with | `"hello".pini_sitelen("lo")` | `kin` |

## String Indexing

```tkp
ijo s = "tkptext"
toki(s[0])    // tkp
toki(s[1])    // text
```

## String Iteration

```tkp
sin text insa "tkptext" {
    toki(text)
}
```

## Concatenation

```tkp
ijo full = "toki" + "hello"
toki(full)    // toki
```
