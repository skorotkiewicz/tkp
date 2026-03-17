# Format Strings

## Positional Arguments

```
sitelen_pali("name: {0}, age: {1}", "name_A", 30)
// → "name: name_A, age: 30"
```

## Named Arguments (from scope)

```
ijo name = "name_A"
ijo age = 30
sitelen_pali("name: {name}, age: {age}")
// → "name: name_A, age: 30"
```

Named mode substitutes `{ijoname}` with the variable's value from the current scope.
