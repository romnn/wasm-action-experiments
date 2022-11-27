just some experiments for now

#### Previous work:

GitHub actions API in rust:

- https://crates.io/crates/gha-toolkit (no github?)
- https://crates.io/crates/actions-toolkit-sys (unmaintained)
- https://crates.io/crates/actions-toolkit (unmaintained but more complete)

Why node + WASM is not ideal:
+ can make network requests using wasm-bindgen + reqwest fetch polyfill (at leatst on modern node with the experimental fetch API)
- cannot by itself access files
+ async support
- single threaded, just like node

Why wasmtime + WASI is not ideal:
+ can access files in a platform independent way
- networking is not yet part of WASI
    - https://github.com/tinygo-org/tinygo/issues/2704
    - https://github.com/bytecodealliance/wasmtime/issues/70
