# Structs

## Definition

```
kulupu person {
    name: sitelen,
    age: nanpa_kind
}
```

## Instantiation

```
ijo name_A = person { name: "name_A", age: 30 }
```

## Field Access

```
toki(name_A.name)    // name_A
toki(name_A.age)    // 30
```

## Field Mutation

```
name_A.age = 31
```

## Nested Structs

```
kulupu addr { city: sitelen }
kulupu staff { name: sitelen, addr: addr }

ijo p = staff { name: "name_B", addr: addr { city: "Seoul" } }
toki(p.addr.city)        // Seoul
p.addr.city = "Busan"     // nested mutation
```

## Impl Blocks (Methods)

```
ken person {
    pali intro(self: person) {
        toki(sitelen_pali("{0}, {1}", self.name, self.age))
    }
}

name_A.intro()    // name_A, 30
```

`self` is the self parameter — refers to the struct instance.
