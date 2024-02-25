#!/bin/bash

echo "Building WASM bundle..."
wasm-pack build --out-name uwu --target web

echo "Renaming package..."
sed -i 's/"name": "uwu_wasm"/"name": "uwu-rs"/' pkg/package.json

echo "Done!"
