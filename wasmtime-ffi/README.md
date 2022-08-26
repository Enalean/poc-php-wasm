# wasmtime-ffi
PHP code that interacts with our [wasmtime wrapper library](../wasmtime-wrapper-lib) to execute wasm binaries. \
It accepts `.wat`, `.wasm` and precompiled `.cwasm` files.

## Run our code
- install php, php-cli, php-ffi
- Add ffi.enable=true to your php.ini
- Check that the module is installed using `php --ri ffi`

First you need to build our library in the the [wasmtime wrapper library](../wasmtime-wrapper-lib) folder, to do so follow the [README.md](../wasmtime-wrapper-lib/README.md) in the corresponding folder.
After that you can execute a (compatible) WebAssembly module with an input JSON file :
- `php ffi-wasmtime.php <filename> <JSON>`

For example you can execute our compatible wasm program `add-json-rs` (remember to build it first !):
- `> php ffi-wasmtime.php ../add-json-rs/target/wasm32-wasi/debug/add-json-rs.wasm json_input/work.json`