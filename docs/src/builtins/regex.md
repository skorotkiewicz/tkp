# Regex

Powered by Rust's `regex` crate.

## Find All Matches

```
ijo result = nasin_alasa("[0-9]+", "abc 123 def 456")
toki(result)    // [123, 456]
```

## Test Match

```
toki(nasin_sama("^[0-9]+$", "12345"))    // kin
toki(nasin_sama("^[0-9]+$", "abc"))      // ala
```

## Replace

```
ijo result = nasin_ante("[0-9]+", "before: 010-1234-5678", "***")
toki(result)    // before: ***-***-***
```

## Functions

| Function | Description |
|----------|-------------|
| `nasin_alasa(pattern, text)` | Find all matches → array of strings |
| `nasin_sama(pattern, text)` | Test if pattern matches → bool |
| `nasin_ante(pattern, text, rep)` | Replace all matches → string |
