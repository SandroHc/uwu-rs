use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uwuify(input: &str) -> String {
    uwu::uwuify(input)
}
