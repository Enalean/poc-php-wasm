# add-json-rs
Rust add program made to be compiled to WebAssembly.
It is a demo project that is meant to be used with [wasmtime-ffi](../wasmtime-ffi/ffi-wasmtime.php)

## How does it work
This program will be fed a JSON file by the host to its memory, the file will look like:
```JSON
{
    "number1":<int>,
    "number2":<int>,
    "res":0
}
```
It will add the fields "number1" and "number2" and put the result in the "res" field. \
Finaly it will return the result as a string to the host.

## Build and execute
Build:
- `> cargo build --target wasm32-wasi`

If you want to precompile the .wasm (You need to install Wasmtime first) you can also do:
- `> wasmtime compile target/wasm32-wasi/debug/add-json-rs.wasm --epoch-interruption`

To run it use [wasmtime-ffi](../wasmtime-ffi/ffi-wasmtime.php)
- `> cd ../wasmtime-ffi`
- `> php ffi-wasmtime.php ../add-json-rs/target/wasm32-wasi/debug/add-json-rs.wasm json_input/work.json`