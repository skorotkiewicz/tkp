# Hello World

Create a file called `hello.tkp`:

```tkp
toki("toki a, ma o!")
```

Run it:

```bash
tkp interpret hello.tkp
```

Output:
```
toki a, ma o!
```

## With a Function

```tkp
pali main() {
    toki("Hello from TKP!")
}

main()
```

## Compile to Binary

```bash
tkp build hello.tkp    # creates ./hello binary
./hello                 # runs natively
```
