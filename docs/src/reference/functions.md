# Functions

## Declaration

```tkp
pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    pana a + b
}
```

## Calling

```tkp
ijo result = add(3, 4)
toki(result)    // 7
```

## No Return Type (void)

```tkp
pali greet(name: sitelen) {
    toki(sitelen_pali("toki, {0}!", name))
}
```

## Recursion

```tkp
pali fibonacci(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 {
        pana n
    }
    pana fibonacci(n - 1) + fibonacci(n - 2)
}
```

## Generics (syntax only)

```tkp
pali first<T>(arr: [T]) -> T {
    pana arr[0]
}
```

Type parameters are parsed but erased at runtime.

## Function Type Parameter

```tkp
pali apply(f: pali, x: nanpa_kind) -> nanpa_kind {
    pana f(x)
}
```
