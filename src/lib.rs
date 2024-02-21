mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uwuify(input: &str) -> String {
    return input.to_string().replace(" n", "ny")
}
