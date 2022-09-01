#! /usr/bin/env bash

set -euxo pipefail

rustup default 1.63.0
rustup target add wasm32-wasi

cargo build --release --target wasm32-wasi --manifest-path=pass-rs/Cargo.toml

mkdir -p dist/
cp -f pass-rs/target/wasm32-wasi/release/pass-rs.wasm dist/example.wasm


wasmedgec dist/example.wasm dist/example-wasmedge-aot.wasm
wasmer compile --cranelift dist/example.wasm -o dist/example-wasmer-aot-cranelift.wasmu
wasmer compile --singlepass dist/example.wasm -o dist/example-wasmer-aot-singlepass.wasmu
wasmer compile --llvm dist/example.wasm -o dist/example-wasmer-aot-llvm.wasmu
wasmtime compile -o dist/example-wasmtime-aot.wasm dist/example.wasm

hyperfine -N --warmup 3 --runs 1000 --export-markdown=benchmark-result.md --export-json=benchmark-result.json \
    'wasmedge dist/example.wasm' \
    'wasmedge dist/example-wasmedge-aot.wasm' \
    'wasmer run --cranelift --disable-cache dist/example.wasm' \
    'wasmer run --singlepass --disable-cache dist/example.wasm' \
    'wasmer run --llvm --disable-cache dist/example.wasm' \
    'wasmer run --cranelift --disable-cache dist/example-wasmer-aot-cranelift.wasmu' \
    'wasmer run --singlepass --disable-cache dist/example-wasmer-aot-singlepass.wasmu' \
    'wasmer run --llvm --disable-cache dist/example-wasmer-aot-llvm.wasmu' \
    'wasmtime --disable-cache dist/example.wasm' \
    'wasmtime --disable-cache --allow-precompiled dist/example-wasmtime-aot.wasm'
