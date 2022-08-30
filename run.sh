#! /usr/bin/env bash

set -euxo pipefail

rustup default 1.63.0
rustup target add wasm32-wasi

pushd wasmtime-wrapper-lib

cargo build --release

popd

pushd add-json-rs 

cargo build --release --target wasm32-wasi
../optimize-wasm/optimize-wasm.sh target/wasm32-wasi/release/add-json-rs.wasm

popd

pushd wasmtime-ffi

php ffi-wasmtime.php ../add-json-rs/add-json-rs-optimized.cwasm json_input/fatwork.json

popd