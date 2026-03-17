# Tuples

## Creating Tuples

```
ijo coord = (10, 20)
ijo person = ("name_A", 30, kin)
```

## Access by Index

```
toki(coord.0)    // 10
toki(coord.1)    // 20
toki(person.0)    // name_A
```

## As Return Type

```
pali minmax(arr: [nanpa_kind]) -> (nanpa_kind, nanpa_kind) {
    ijo min = arr[0]
    ijo max = arr[0]
    sin v insa arr {
        la v < min { min = v }
        la v > max { max = v }
    }
    pana (min, max)
}

ijo result = minmax([5, 1, 9, 3, 7])
toki(sitelen_pali("min: {0}, max: {1}", result.0, result.1))
```
