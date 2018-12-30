# JS Sound Gen

This is a WASM/JS tool prototype for generating sound in browser in real time. For now, it produces a hard-coded sequence of sounds, generatid sequentially after page startup.

## Building

Building of the project consist of two parts: creating WASM code and building the webpage itself.

### WASM code

You'll need nightly Rust (tested on versions 1.31 and newer) and a pair of tools, namely `wasm-pack` and `wasm-bindgen`.
To install nightly Rust, use:
```
rustup toolchain add nightly
```
Tools are instaled by running:
```
cargo install wasm-bindgen
cargo install wasm-pack
```
To build thw WASM module, use:
```
wasm-pack build
```

### Webpage

Webpage source code is placed under `www` folder as an NPM package. You should run `npm install` inside it, and then use `npm start` to launch development server.
If you're editing the code, you must run `tsc` manually. If the dev server is running, it will get the changes in transpiled code automatically.

## TODOs

- Implement sound generation with dynamially changing length
- Implement different sound tones
- Save all the generated sound into one WAV blob
- Make it possible to build site in production mode

## Contributing

Issues and PRs are welcomed.

## License

MIT
