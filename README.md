# Untrusted code execution in PHP with FFI and WebAssembly
This repository host our proof of code to answer the question *How to execute untrusted code on our PHP based platform in a secure and timely manner ?*

## What are all these folders
- [wasmtime-ffi](wasmtime-ffi): This is the PHP code that interacts with wasmtime-wrapper-lib to execute wasm binaries.
- [wasmtime-wrapper-lib](wasmtime-wrapper-lib): This is a wrapper of the Wasmtime Rust library that we've written in order to simplify the interactions between PHP and Wasmtime.
- [add-json-rs](add-json-rs/src/main.rs): A rust program that we compile to WebAssembly and execute through ffi-wasmtime.php.
It is our example WASM module to prove that everything is working correctly.

## Building and running
<<<<<<< HEAD
To build and run our code please follow the individual README.md in each folder, you can start with [the one in wasmtime-ffi](wasmtime-ffi/README.md)

A [Nix shell](https://nixos.org/manual/nix/stable/command-ref/nix-shell.html) is provided to get access to the required development tools.
If you want to build everything and execute the POC you can run `nix-shell --run './run.sh'`
=======
- Install [NixOS](https://nixos.org/download.html)
- `./run.sh`
>>>>>>> 549e42f (Update README.md with new launching instructions)
