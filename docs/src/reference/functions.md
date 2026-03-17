# Functions

## Declaration

```
pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind {
    pana a + b
}
```

## Calling

```
ijo result = add(3, 4)
toki(result)    // 7
```

## No Return Type (void)

```
pali greet(name: sitelen) {
    toki(sitelen_pali("toki, {0}!", name))
}
```

## Recursion

```
pali fibonacci(n: nanpa_kind) -> nanpa_kind {
    la n <= 1 {
        pana n
    }
    pana fibonacci(n - 1) + fibonacci(n - 2)
}
```

## Generics (syntax only)

```
pali first<T>(arr: [T]) -> T {
    pana arr[0]
}
```

Type parameters are parsed but erased at runtime.

## Function Type Parameter

```
pali apply(f: pali, x: nanpa_kind) -> nanpa_kind {
    pana f(x)
}
```
