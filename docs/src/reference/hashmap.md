# HashMap / Dictionary

## Creating a Map

```tkp
ijo score = lipu("math", 95, "eng", 88, "sci", 92)
ijo empty_map = lipu()
```

Arguments are key-value pairs: `lipu(key1, val1, key2, val2, ...)`.

## Access and Mutation

```tkp
toki(score["math"])       // 95
score["lang"] = 100       // add new key
score["math"] = 99        // update existing
```

## Methods

| Method | Description | Example |
|--------|-------------|---------|
| `.nimi_ale()` | All keys as array | `score.nimi_ale()` → `["math", "eng", "sci"]` |
| `.ijo_ale()` | All values as array | `score.ijo_ale()` → `[95, 88, 92]` |
| `.suli_ijo()` | Number of entries | `score.suli_ijo()` → `3` |
| `.jo(key)` | Key exists | `score.jo("math")` → `kin` |
| `.pakala(key)` | Remove key | `score.pakala("eng")` |

## Iteration

```tkp
ijo keys = score.nimi_ale()
sin key insa keys {
    toki(sitelen_pali("{0}: {1}", key, score[key]))
}
```
