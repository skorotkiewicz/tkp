# HTTP

Powered by Rust's `reqwest` crate (blocking mode).

## GET Request

```tkp
ijo resp = tawa_kama("https://httpbin.org/get")
toki(resp)
```

## POST Request

```tkp
ijo body = kulupu_pali(lipu("name", "name_A"))
ijo resp = tawa_pana("https://httpbin.org/post", body)
toki(resp)
```

POST sends with `Content-Type: application/json`. If the body is not a string, it's auto-converted to JSON.

## Error Handling

```tkp
lukin {
    ijo resp = tawa_kama("https://invalid-url.example")
} alasa(error) {
    toki(sitelen_pali("HTTP error: {0}", error))
}
```

## Functions

| Function | Description |
|----------|-------------|
| `tawa_kama(url)` | GET request → response body as string |
| `tawa_pana(url, body)` | POST request with JSON body → response as string |
