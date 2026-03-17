# Control Flow

## If / Else-If / Else

```tkp
la score >= 90 {
    toki("A")
} ante la score >= 80 {
    toki("B")
} ante {
    toki("C")
}
```

## For Loop

```tkp
sin ijo i = 0; i < 10; i += 1 {
    toki(i)
}
```

## For-In Loop

Iterate over arrays:
```tkp
sin fruit insa ["apple", "pear", "grape"] {
    toki(fruit)
}
```

Iterate over strings:
```tkp
sin text insa "tkptext" {
    toki(text)    // tkp, text
}
```

Iterate over ranges:
```tkp
sin i insa 0..5 {
    toki(i)    // 0, 1, 2, 3, 4
}
```

## While Loop

```tkp
ijo n = 0
lon n < 5 {
    toki(n)
    n += 1
}
```

## Break and Continue

```tkp
sin i insa 0..100 {
    la i == 50 { pini }
    la i % 2 == 0 { tawa }
    toki(i)
}
```

## Range Operator

```tkp
ijo range = 0..10     // creates [0, 1, 2, ..., 9]
ijo suli_ijo = range.suli_ijo()  // 10
```
