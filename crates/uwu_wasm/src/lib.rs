use uwu::{Uwu, UwuBuilder};
use wasm_bindgen::prelude::*;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Options to control the behaviour of the uwu algorithm.
#[wasm_bindgen]
#[derive(Clone)]
pub struct UwuOptions {
    /// Enables the lowercase feature, e.g. 'Hello' becomes 'hello'. Beware that the other features
    /// may misbehave if this feature is disabled.
    pub lowercase: bool,
    /// Enables expression replacement, e.g. 'what' becomes 'nani'.
    pub expressions: bool,
    /// Enables replacement of 'l' and 'r' with 'w', e.g. 'lovely' becomes 'wovewy'.
    pub w_replace: bool,
    /// Enables stutter, e.g. 'hello' becomes 'h-hello'.
    pub stutter: bool,
    /// Specifies how frequent the stutter is. A value of 1 will add stutter to every word, whereas
    /// a value of 2 will add stutter every 2 words on average.
    pub stutter_chance: u8,
    /// Enables adding emojis after punctuation, e.g. 'goodbye.' becomes 'goodbye. OwO'.
    pub emojis: bool,
    /// Specifies how frequently emojis are added. A value of 1 will add emojis after every
    /// punctuation, whereas a value of 2 will add emojis every 2 punctuation marks on average.
    pub emojis_chance: u8,
}

impl From<UwuOptions> for UwuBuilder {
    fn from(options: UwuOptions) -> Self {
        let mut builder = Self::new();

        if options.lowercase {
            builder.lowercase();
        }
        if options.expressions {
            builder.expressions();
        }
        if options.w_replace {
            builder.w_replace();
        }
        if options.stutter {
            builder.stutter(options.stutter_chance);
        }
        if options.emojis {
            builder.emojis(options.emojis_chance);
        }

        builder
    }
}

impl From<UwuOptions> for Uwu {
    fn from(options: UwuOptions) -> Self {
        UwuBuilder::from(options).build()
    }
}

#[wasm_bindgen]
pub fn uwuify(input: &str, options: Option<UwuOptions>) -> String {
    let uwu = parse_options(options);
    uwu.uwuify(input).unwrap_or(input.to_string())
}

fn parse_options(options: Option<UwuOptions>) -> Uwu {
    if let Some(options) = options {
        Uwu::from(options)
    } else {
        Uwu::new()
    }
}
