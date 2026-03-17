# Error TKPdling

## Try / Catch

```tkp
lukin {
    ijo content = lipu_lukin("missing_file.txt")
    toki(content)
} alasa(error) {
    toki(sitelen_pali("error: {0}", error))
}
```

The error variable (`error`) contains the error message as a string.

## What Gets Caught

- Division by zero
- File not found
- Index out of bounds
- Undefined variable access
- Type mismatches at runtime

## Example: Safe Division

```tkp
pali unsafe_div(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    lukin {
        pana a / b
    } alasa(error) {
        toki(sitelen_pali("div alasa: {0}", error))
        pana 0
    }
}
```
