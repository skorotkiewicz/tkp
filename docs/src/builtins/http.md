# HTTP

Powered by Rust's `reqwest` crate (blocking mode).

## GET Request

```
ijo resp = lipu_lukin_http("https://httpbin.org/get")
toki(resp)
```

## POST Request

```
ijo body = kulupu_pali(dictionary("name", "name_A"))
ijo resp = tawa_pana("https://httpbin.org/post", body)
toki(resp)
```

POST sends with `Content-Type: application/json`. If the body is not a string, it's auto-converted to JSON.

## Error TKPdling

```
lukin {
    ijo resp = lipu_lukin_http("https://invalid-url.example")
} alasa(error) {
    toki(sitelen_pali("HTTP error: {0}", error))
}
```

## Functions

| Function | Description |
|----------|-------------|
| `lipu_lukin_http(url)` | GET request → response body as string |
| `tawa_pana(url, body)` | POST request with JSON body → response as string |
