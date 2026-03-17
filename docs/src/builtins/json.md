# JSON

Powered by Rust's `serde_json` crate.

## Parse JSON

```tkp
ijo text = "{\"name\": \"name_A\", \"age\": 30}"
ijo data = kulupu_lukin(text)
toki(data["name"])    // name_A
toki(data["age"])    // 30
```

JSON objects become `dictionary`, arrays become `array`.

## Generate JSON

```tkp
ijo user = dictionary("name", "name_A", "age", 30)
ijo json = kulupu_pali(user)
toki(json)    // {"age":30,"name":"name_A"}
```

## Pretty Print

```tkp
ijo pretty = kulupu_pona(user)
toki(pretty)
```

## Functions

| Function | Description |
|----------|-------------|
| `kulupu_lukin(sitelen)` | Parse JSON string → TKP value |
| `kulupu_pali(value)` | TKP value → JSON string |
| `kulupu_pona(value)` | TKP value → pretty-printed JSON |
