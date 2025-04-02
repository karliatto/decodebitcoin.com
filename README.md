 DecodeBitcoin.com

This is an experimental project using Rust and WASM to decode Bitcoin-related data.

## Features

- [x] Bitcoin Transaction decoding
- [x] BIP 39 XOR
- [ ] BIP 32
- [ ] BIP 21
- [ ] BOLT 11
- [ ] BOLT 12

## Web Interface

### Development Server

Run the development server with:

```bash
npm run serve
```

Then visit http://localhost:8080 in your browser.

### Production Build

Build and serve the production version:

```bash
npm run build
```

#### Serving Options

1. Using Python's HTTP server:
```bash
cd dist
python3 -m http.server 8080
```

2. Using Nginx:
   - Copy the `dist` directory contents to your web root
   - Add WASM MIME type to your Nginx configuration:
```nginx
types {
    application/wasm wasm;
}
```

## CLI Usage

### Decode Bitcoin Transaction

```bash
cargo run --bin decodebitcoin-cli -- decode <transaction_hex>
```

### Derive from Extended Private Key

```bash
cargo run --bin decodebitcoin-cli -- derive <xpriv>
```

Example:
```bash
cargo run --bin decodebitcoin-cli -- derive tprv8ZgxMBicQKsPczGmwTSuQwPSVoqEyXBxinSRRVHieeF7FUi8eZVh46dRJUSPr8tofmC1TymdPMGYmu6TakaEQaA27VMYZxHs4pcekFTotCC
```