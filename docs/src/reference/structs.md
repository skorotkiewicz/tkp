# Structs

## Definition

```tkp
kulupu person {
    name: sitelen,
    age: nanpa_kind
}
```

## Instantiation

```tkp
ijo name_A = person { name: "name_A", age: 30 }
```

## Field Access

```tkp
toki(name_A.name)    // name_A
toki(name_A.age)    // 30
```

## Field Mutation

```tkp
name_A.age = 31
```

## Nested Structs

```tkp
kulupu addr { city: sitelen }
kulupu staff { name: sitelen, addr: addr }

ijo p = staff { name: "name_B", addr: addr { city: "Seoul" } }
toki(p.addr.city)        // Seoul
p.addr.city = "Busan"     // nested mutation
```

## Impl Blocks (Methods)

```tkp
ken person {
    pali intro(self: person) {
        toki(sitelen_pali("{0}, {1}", self.name, self.age))
    }
}

name_A.intro()    // name_A, 30
```

`self` is the self parameter — refers to the struct instance.
