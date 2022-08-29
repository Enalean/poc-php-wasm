#! /usr/bin/env bash

set -euxo pipefail

rustup default 1.63.0
rustup target add wasm32-wasi

pushd wasmtime-wrapper-lib

cargo build

popd

pushd add-json-rs 

cargo build --target wasm32-wasi
wasmtime compile target/wasm32-wasi/debug/add-json-rs.wasm --epoch-interruption

popd

pushd wasmtime-ffi

php ffi-wasmtime.php ../add-json-rs/add-json-rs.cwasm json_input/work.json

popd