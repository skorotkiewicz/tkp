# HashMap / Dictionary

## Creating a Map

```
ijo score = dictionary("math", 95, "eng", 88, "sci", 92)
ijo empty_map = dictionary()
```

Arguments are key-value pairs: `dictionary(key1, val1, key2, val2, ...)`.

## Access and Mutation

```
toki(score["math"])       // 95
score["lang"] = 100       // add new key
score["math"] = 99        // update existing
```

## Methods

| Method | Description | Example |
|--------|-------------|---------|
| `.keys()` | All keys as array | `score.keys()` → `["math", "eng", "sci"]` |
| `.values()` | All values as array | `score.values()` → `[95, 88, 92]` |
| `.suli_ijo()` | Number of entries | `score.suli_ijo()` → `3` |
| `.contains(key)` | Key exists | `score.contains("math")` → `kin` |
| `.remove(key)` | Remove key | `score.remove("eng")` |

## Iteration

```
ijo keys = score.keys()
sin key insa keys {
    toki(sitelen_pali("{0}: {1}", key, score[key]))
}
```
