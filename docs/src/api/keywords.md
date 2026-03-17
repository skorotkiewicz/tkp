# Keyword Reference

## Control Keywords

| Keyword | English | Syntax |
|---------|---------|--------|
| `pali` | function | `pali name(params: type) -> panatype { }` |
| `pana` | return | `pana value` |
| `ijo` | let (mutable) | `ijo name = value` |
| `awen` | const | `awen name = value` |
| `la` | if | `la condition { }` |
| `ante` | else | `ante { }` or `ante la condition { }` |
| `sin` | for | `sin ijo i = 0; i < n; i += 1 { }` |
| `lon` | while | `lon condition { }` |
| `pini` | break | `pini` |
| `tawa` | continue | `tawa` |
| `insa` | in (for-in) | `sin x insa array { }` |

## Type Keywords

| Keyword | English | LLVM |
|---------|---------|------|
| `nanpa_kind` | int | i64 |
| `kipisi` | float | f64/double |
| `sitelen` | string | i8* |
| `lawa` | bool | i1 |
| `weka` | void/null | void |

## Structure Keywords

| Keyword | English | Syntax |
|---------|---------|--------|
| `kulupu` | struct | `kulupu name { field: type }` |
| `ken` | impl | `ken struct { pali method(self: T) { } }` |
| `nanpa` | enum | `nanpa name { variant1, variant2 }` |
| `lukin` | try | `lukin { } alasa(error) { }` |
| `alasa` | catch | see `lukin` |
| `sama` | match | `sama value { pattern => result }` |
| `jo` | import | `jo "file.tkp"` |

## Literal Keywords

| Keyword | English |
|---------|---------|
| `kin` | true |
| `ala` | false |
| `weka` | null/void |

## Built-in Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `toki(value)` | `(any...) -> weka` | Print to stdout |
| `kute()` | `() -> sitelen` | Read line from stdin |
| `lili_nanpa(x)` | `(number) -> kipisi` | Square root |
| `wawa_nanpa(x)` | `(number) -> number` | Absolute value |
| `suli_nanpa(base, exp)` | `(number, number) -> kipisi` | Power |
| `nanpa_ante(x)` | `(any) -> nanpa_kind` | Convert to integer |
| `kipisi_ante(x)` | `(any) -> kipisi` | Convert to float |
| `suli_ijo(s)` | `(sitelen) -> nanpa_kind` | String length |
| `sitelen_pali(template, ...)` | `(sitelen, any...) -> sitelen` | Format string |
| `lipu_lukin(path)` | `(sitelen) -> sitelen` | Read file |
| `lipu_sitelen(path, content)` | `(sitelen, sitelen) -> weka` | Write file |
| `lipu_sin(path, content)` | `(sitelen, sitelen) -> weka` | Append to file |
| `lipu_lon(path)` | `(sitelen) -> lawa` | File exists |
| `toki_pakala(value)` | `(any...) -> weka` | Print to stderr |
