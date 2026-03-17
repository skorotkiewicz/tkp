# Data Types

## Primitive Types

| Type | TKP | Description | Examples |
|------|-----|-------------|----------|
| Integer | `nanpa_kind` | 64-bit signed integer | `42`, `-10`, `0` |
| Float | `kipisi` | 64-bit floating point | `3.14`, `-0.5` |
| String | `sitelen` | UTF-8 string | `"toki"` |
| Boolean | `lawa` | true/false | `kin`, `ala` |
| Void | `weka` | null/void | `weka` |

## Type Coercion

Int and Float mix automatically:

```
toki(1 + 1.5)     // 2.5 (Float)
toki(3 * 2.0)     // 6.0 (Float)
toki(10 - 0.5)    // 9.5 (Float)
```

## Compound Types

- **Arrays**: `[1, 2, 3]` — type `[nanpa_kind]`
- **Tuples**: `(1, "hello", kin)` — type `(nanpa_kind, sitelen, lawa)`
- **Structs**: `kulupu person { name: sitelen }`
- **Enums**: `nanpa direction { up, down }`
