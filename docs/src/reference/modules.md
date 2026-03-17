# Modules

## Importing Files

```
jo "math_tool.tkp"
```

This executes the file and imports all its definitions (functions, variables, structs) into the current scope.

## Example

`math_tool.tkp`:
```
pali maxvalue(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    la a > b { pana a }
    pana b
}
```

`main.tkp`:
```
jo "math_tool.tkp"
toki(maxvalue(10, 20))    // 20
```
