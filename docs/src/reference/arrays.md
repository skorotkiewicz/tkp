# Arrays

## Creating Arrays

```tkp
ijo number = [1, 2, 3, 4, 5]
ijo empty_arr = []
```

## Indexing

```tkp
toki(number[0])     // 1
toki(number[-1])    // 5 (negative indexing)
number[0] = 99      // mutation
```

## Methods

| Method | Description | Example |
|--------|-------------|---------|
| `.sin_ijo(val)` | Append element | `arr.sin_ijo(6)` |
| `.pakala(idx)` | Remove at index | `arr.pakala(0)` |
| `.suli_ijo()` | Length | `arr.suli_ijo()` → `5` |
| `.jo(val)` | Contains | `arr.jo(3)` → `kin` |
| `.monsi()` | Reverse (new array) | `arr.monsi()` |
| `.nasin_ijo()` | Sort (new array) | `arr.nasin_ijo()` |
| `.wan(sep)` | Join to string | `arr.wan(", ")` |

## Iteration

```tkp
sin item insa [1, 2, 3] {
    toki(item)
}
```
