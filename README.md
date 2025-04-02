# DecodeBitcoin.com

This is a experimental project using Rust and WASM to decode Bitcoin.

- [x] Bitcoin Transactions decoding
- [x] BIP 39 XOR
- [ ] BIP 32
- [] Bip21
- [] bolt11
- [] bolt12
  ...

## Run web server with WASM

You can build the example locally with:

```bash
npm run serve
```

and then visiting http://localhost:8080 in a browser should run the example!


## Build and run CLI

```
cargo run --bin decodebitcoin-cli -- <transaction_hex>
```