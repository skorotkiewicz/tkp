# Closures

## Anonymous Functions

```tkp
ijo double = pali(x: nanpa_kind) { pana x * 2 }
toki(double(5))    // 10
```

## Environment Capture

Closures capture variables from their enclosing scope:

```tkp
ijo pearnum = 3
ijo mul = pali(x: nanpa_kind) { pana x * pearnum }
toki(mul(5))    // 15
```

## Passing as Arguments

```tkp
pali apply(f: pali, x: nanpa_kind) -> nanpa_kind {
    pana f(x)
}

ijo sq = pali(x: nanpa_kind) { pana x * x }
toki(apply(sq, 4))    // 16
```
