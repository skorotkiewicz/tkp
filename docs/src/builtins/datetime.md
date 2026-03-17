# Date & Time

Powered by Rust's `chrono` crate.

## Current Time

```
toki(tenpo_ni())    // 2025-03-15 12:30:45
toki(suno_ni())    // 2025-03-15
toki(nanpa_tenpo())  // 1710500000 (Unix timestamp)
```

## Functions

| Function | Return | Example |
|----------|--------|---------|
| `tenpo_ni()` | `sitelen` | `"2025-03-15 12:30:45"` |
| `suno_ni()` | `sitelen` | `"2025-03-15"` |
| `nanpa_tenpo()` | `nanpa_kind` | `1710500000` |

`tenpo_ni()` and `suno_ni()` use local time. `nanpa_tenpo()` returns UTC Unix timestamp.
