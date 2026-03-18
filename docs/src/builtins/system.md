# System & Process

## Shell Command

```tkp
ijo result = lawa_pali("ls -la")
toki(result)
```

Runs the command via `sh -c` and returns stdout as a string.

## Environment Variables

```tkp
ijo home = ma_ijo("HOME")
toki(home)    // /Users/username

ijo missingijo = ma_ijo("NONEXISTENT")
toki(missingijo)    // weka
```

Returns `weka` if the variable doesn't exist.

## CLI Arguments

```tkp
ijo args = toki_ijo()
sin arg insa args {
    toki(arg)
}
```

Returns arguments passed after the filename: `tkp interpret file.tkp arg1 arg2`

## Sleep

```tkp
lape(1000)    // sleep 1 second (1000 milliseconds)
```

## Type Introspection

```tkp
toki(nasin(42))          // nanpa_kind
toki(nasin("hello"))     // sitelen
toki(nasin([1,2,3]))     // kulupu_kipisi
toki(nasin(lipu()))      // lipu
toki(nasin(kin))         // lawa
```

## Functions

| Function | Description |
|----------|-------------|
| `lawa_pali(cmd)` | Run shell command → stdout string |
| `ma_ijo(name)` | Get env var → string or weka |
| `toki_ijo()` | CLI args → array of strings |
| `lape(ms)` | Sleep for N milliseconds |
| `nasin(value)` | Type name → string |
| `toki_pakala(value)` | Print to stderr |
