# Pattern Matching

## Basic Match

```tkp
sama value {
    1 => toki("one")
    2 => toki("two")
    _ => toki("other")
}
```

## With Blocks

```tkp
sama status {
    "active" => {
        toki("active status")
        alasa_pali()
    }
    "nonactive" => toki("nonactive")
    _ => toki("unknown")
}
```

## Pattern Types

| Pattern | Example | Description |
|---------|---------|-------------|
| Integer | `42` | Matches exact integer |
| String | `"hello"` | Matches exact string |
| Boolean | `kin`, `ala` | Matches boolean |
| Wildcard | `_` | Matches anything |
| Binding | `x` | Matches anything, binds to variable `x` |
| Array | `[1, 2, 3]` | Matches array structure |

## Variable Binding

```tkp
sama value {
    0 => toki("zero")
    n => toki(sitelen_pali("value: {0}", n))
}
```
