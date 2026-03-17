# Type Conversion

| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `nanpa_ante(x)` | Convert to integer | `nanpa_ante("42")` | `42` |
| `kipisi_ante(x)` | Convert to float | `kipisi_ante(42)` | `42.0` |
| `suli_ijo(s)` | String length | `suli_ijo("tkptext")` | `2` |

## Conversion Rules

```tkp
nanpa_ante(3.14)      // 3 (truncates)
nanpa_ante("42")      // 42 (parse string)
nanpa_ante(kin)        // 1
nanpa_ante(ala)      // 0

kipisi_ante(42)        // 42.0
kipisi_ante("3.14")    // 3.14
```
