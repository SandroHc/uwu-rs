use wasm_bindgen::prelude::*;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn uwuify(input: &str) -> String {
    uwu::uwuify(input).unwrap_or(input.to_string())
}
