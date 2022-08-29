# wasmtime-wrapper-lib
Our wrapper Rust library for wasmtime. \
This library only expose one function in order to make it much easier to interact with PHP's broken parser.
It accepts `.wat`, `.wasm` and precompiled `.cwasm` files.

## Run our code
- Install the Rust toolchain on your machine.
- `cargo build`

You can directly run compatible WASM module by doing:
- `cargo run <filename> <JSON>`

Please note that it can and should be used through [wasmtime-ffi](../wasmtime-ffi/ffi-wasmtime.php) and with [add-json-rs](../add-json-rs/)