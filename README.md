# DecodeBitcoin.com

[Scaffolding]: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

## Develop

```
$ wasm-pack build --target web
```

Then serve this directory in your favorite webserver and navigate to `host:port`
to open the index.html in your browser:

```
# static server from https://crates.io/crates/https
http

# or use python
python2 -m SimpleHTTPServer
python3 -m http.server
```