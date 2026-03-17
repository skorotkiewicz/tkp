# System & Process

## Shell Command

```
ijo result = lawa_pali("ls -la")
toki(result)
```

Runs the command via `sh -c` and returns stdout as a string.

## Environment Variables

```
ijo home = ma_ijo("HOME")
toki(home)    // /Users/username

ijo missingijo = ma_ijo("NONEXISTENT")
toki(missingijo)    // weka
```

Returns `weka` if the variable doesn't exist.

## CLI Arguments

```
ijo args = toki_ijo()
sin arg insa args {
    toki(arg)
}
```

Returns arguments passed after the filename: `tkp interpret file.tkp arg1 arg2`

## Sleep

```
lape(1000)    // sleep 1 second (1000 milliseconds)
```

## Type Introspection

```
toki(type(42))          // nanpa_kind
toki(type("hello"))     // sitelen
toki(type([1,2,3]))     // array
toki(type(dictionary()))      // dictionary
toki(type(kin))          // lawa
```

## Functions

| Function | Description |
|----------|-------------|
| `lawa_pali(cmd)` | Run shell command → stdout string |
| `ma_ijo(name)` | Get env var → string or weka |
| `toki_ijo()` | CLI args → array of strings |
| `lape(ms)` | Sleep for N milliseconds |
| `type(value)` | Type name → string |
| `toki_pakala(value)` | Print to stderr |
