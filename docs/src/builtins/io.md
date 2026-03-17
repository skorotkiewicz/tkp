# I/O

## Output

```tkp
toki("toki")        // print to stdout
toki(42)                 // prints any value
toki("name:", name)       // multiple args, space-separated
toki_pakala("error message")    // print to stderr
```

## Input

```tkp
ijo name = kute()
toki(sitelen_pali("toki, {0}!", name))
```

`kute()` reads one line from stdin and returns it as a string.
