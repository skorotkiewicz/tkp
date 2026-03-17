# Enums

## Definition

```
nanpa direction {
    up,
    down,
    left,
    right
}
```

## Access

Variants are accessed with `::`:

```
toki(direction::up)       // 0
toki(direction::down)     // 1
toki(direction::right)   // 3
```

Variants are integer values starting from 0.

## Pattern Matching with Enums

```
sama direction::down {
    0 => toki("up")
    1 => toki("down")
    _ => toki("other")
}
```
