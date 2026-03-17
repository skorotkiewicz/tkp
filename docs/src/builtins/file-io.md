# File I/O

| Function | Description | Example |
|----------|-------------|---------|
| `lipu_lukin(path)` | Read file to string | `lipu_lukin("data.txt")` |
| `lipu_sitelen(path, content)` | Write string to file | `lipu_sitelen("out.txt", "hello")` |
| `lipu_sin(path, content)` | Append to file | `lipu_sin("log.txt", "line\n")` |
| `lipu_lon(path)` | Check if file exists | `lipu_lon("data.txt")` → `kin`/`ala` |

## Example: Read and Process

```tkp
lukin {
    ijo content = lipu_lukin("data.txt")
    ijo lines = content.split("\n")
    sin line insa lines {
        toki(line)
    }
} alasa(error) {
    toki(sitelen_pali("file error: {0}", error))
}
```
