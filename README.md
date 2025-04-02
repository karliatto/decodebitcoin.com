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

* Decode Bitcoin Transaction

```
cargo run --bin decodebitcoin-cli -- decode <transaction_hex>
```

* Derive from an xpriv

```
cargo run --bin decodebitcoin-cli -- derive <xpriv>
```

Like: 

```
cargo run --bin decodebitcoin-cli -- derive tprv8ZgxMBicQKsPczGmwTSuQwPSVoqEyXBxinSRRVHieeF7FUi8eZVh46dRJUSPr8tofmC1TymdPMGYmu6TakaEQaA27VMYZxHs4pcekFTotCC
```
