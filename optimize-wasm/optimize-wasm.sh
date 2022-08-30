#! /usr/bin/env bash
if [ -z "$1" ]
  then
    echo "Usage: $0 <filename.wasm>"
    exit
fi

new="$(basename "$1" .wasm)"-optimized.wasm

wasm-opt -O4 "$1" -o "$new"
wasmtime compile --epoch-interruption "$new"
