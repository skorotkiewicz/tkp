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
| `.push(value)` | Append element | `arr.push(6)` |
| `.remove(index)` | Remove at index | `arr.remove(0)` |
| `.suli_ijo()` | Length | `arr.suli_ijo()` → `5` |
| `.contains(value)` | Contains | `arr.contains(3)` → `kin` |
| `.reverse()` | Reverse (new array) | `arr.reverse()` |
| `.sort()` | Sort (new array) | `arr.sort()` |
| `.join(sep)` | Join to string | `arr.join(", ")` |

## Iteration

```tkp
sin item insa [1, 2, 3] {
    toki(item)
}
```
