@ECHO OFF

ECHO "Building WASM bundle..."
CALL wasm-pack build --out-name uwu --target web

ECHO "Renaming package..."
REM sed -i 's/"name": "uwu_wasm"/"name": "uwu-rs"/' pkg/package.json

ECHO "Done!"
