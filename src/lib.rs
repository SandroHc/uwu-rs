use aho_corasick::{AhoCorasick, AhoCorasickBuilder, Input};
use std::io::{Cursor, Write};
use thiserror::Error;

const EXPRESSION_PATTERNS: [&str; 7] = ["small", "cute", "fluff", "love", "stupid", "what", "meow"];
const EXPRESSION_PATTERNS_REPLACE: [&str; 7] =
    ["smol", "kawaii~", "floof", "luv", "baka", "nani", "nya~"];

const NYA_PATTERNS: [&str; 3] = [" n", "\nn", "\tn"];
const NYA_PATTERNS_REPLACE: [&str; 3] = [" ny", " ny", " ny"];

const PUNCTUATION_PATTERNS: [&str; 3] = [", ", ". ", "! "];

const EMOJIS: [&str; 31] = [
    "rawr x3 ",
    "OwO ",
    "UwU ",
    "o.O ",
    "-.- ",
    ">w< ",
    "(⑅˘꒳˘) ",
    "(ꈍᴗꈍ) ",
    "(˘ω˘) ",
    "(U ᵕ U❁) ",
    "σωσ ",
    "òωó ",
    "(///ˬ///✿) ",
    "(U ﹏ U) ",
    "( ͡o ω ͡o ) ",
    "ʘwʘ ",
    ":3 ",
    ":3 ", // important enough to have twice
    "XD ",
    "nyaa~~ ",
    "mya ",
    ">_< ",
    "😳 ",
    "🥺 ",
    "😳😳😳 ",
    "rawr ",
    "^^ ",
    "^•ﻌ•^ ",
    "/(^•ω•^) ",
    "(✿oωo) ",
    "👉👈",
];

/// A uwu instance capable of running the uwu algorithm.
///
/// The simplest way to use it is:
/// ```
/// let uwuified = uwu::Uwu::new().uwuify("Hello world!");
/// ```
///
/// But you can also modify its behaviour with:
/// ```
/// let uwu = uwu::Uwu {
///     lowercase: false,
///     expressions: false,
///     nya: false,
///     w_replace: false,
///     stutter: false,
///     stutter_chance: 0,
///     emojis: false,
///     emojis_chance: 0,
/// };
/// let uwuified = uwu.uwuify("Hello world!");
/// ```
#[derive(Debug)]
pub struct Uwu {
    /// Enables the lowercase feature, e.g. 'Hello' becomes 'hello'. Beware that the other features
    /// may misbehave if this feature is disabled.
    pub lowercase: bool,
    /// Enables expression replacement, e.g. 'what' becomes 'nani'.
    pub expressions: bool,
    /// Enables nyanification. Words started with 'n' will be appended a 'y', e.g. 'nanomachines'
    /// becomes 'nyanomachines'.
    pub nya: bool,
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

impl Default for Uwu {
    fn default() -> Self {
        Self {
            lowercase: true,
            expressions: true,
            nya: true,
            w_replace: true,
            stutter: true,
            stutter_chance: 4,
            emojis: true,
            emojis_chance: 1,
        }
    }
}

impl Uwu {
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the input text into an uwuified version.
    ///
    /// Example:
    /// ```
    /// let uwuified = uwu::Uwu::new().uwuify("Hello world!");
    /// ```
    pub fn uwuify<S: AsRef<str>>(&self, input: S) -> Result<String, UwuError> {
        let mut rng = fastrand::Rng::with_seed(75777521); // 'uwu!' = 75 77 75 21
        let input = " ".to_string() + &input.as_ref().to_ascii_lowercase() + " ";

        // Expressions
        let matcher_expr = AhoCorasick::new(EXPRESSION_PATTERNS)?;
        let mut out_expr = Vec::with_capacity(input.len());
        matcher_expr.try_stream_replace_all(
            input.as_bytes(),
            &mut out_expr,
            EXPRESSION_PATTERNS_REPLACE.as_slice(),
        )?;

        // Nya-ify
        let matcher_nya = AhoCorasick::new(NYA_PATTERNS)?;
        let mut out_nya = Vec::with_capacity(out_expr.len());
        matcher_nya.try_stream_replace_all(
            Cursor::new(out_expr),
            &mut out_nya,
            NYA_PATTERNS_REPLACE.as_slice(),
        )?;

        // Replace 'l' and 'r' with 'w'
        out_nya.iter_mut().for_each(|byte| {
            if matches!(byte, b'l' | b'r') {
                *byte = b'w';
            }
        });

        // Stutter
        let out_stutter = if out_nya.len() < 2 {
            out_nya
        } else {
            let mut buf = Vec::with_capacity(out_nya.len());
            let mut prev_idx = 0;
            for mut idx in 0..out_nya.len() - 1 {
                if out_nya[idx] == b' '
                    && out_nya[idx + 1].is_ascii_alphabetic()
                    && rng.u8(0..self.stutter_chance) == 0
                {
                    idx += 1;
                    let section = &out_nya[prev_idx..idx];
                    let ch = out_nya[idx];
                    buf.write_all(section)?;
                    buf.write_all(&[ch])?;
                    buf.write_all(&[b'-'])?;
                    prev_idx = idx;
                }
            }
            // Dump remaining
            buf.write_all(&out_nya[prev_idx..])?;
            buf
        };

        // Emojis
        let matcher_punct = AhoCorasickBuilder::new().build(PUNCTUATION_PATTERNS)?;
        let matches = matcher_punct
            .try_find_iter(Input::new(&out_stutter))?
            .map(|mat| mat.end())
            .collect::<Vec<usize>>();
        let out_emojis = if matches.is_empty() {
            out_stutter
        } else {
            let mut buf = Vec::with_capacity(out_stutter.len());
            let mut prev_idx = 0;
            for idx in matches {
                if rng.u8(0..self.emojis_chance) != 0 {
                    continue;
                }
                let section = &out_stutter[prev_idx..idx];
                let emoji = rng.choice(EMOJIS).unwrap_or("uwu");
                buf.write_all(section)?;
                buf.write_all(emoji.as_bytes())?;
                prev_idx = idx;
            }
            // Dump remaining
            buf.write_all(&out_stutter[prev_idx..])?;
            buf
        };

        let converted = String::from_utf8_lossy(&out_emojis).trim().to_string();
        Ok(converted)
    }
}

#[derive(Error, Debug)]
pub enum UwuError {
    #[error("string matcher build error: {0}")]
    StringMatcherBuild(#[from] aho_corasick::BuildError),
    #[error("string matcher match error: {0}")]
    StringMatcherMatch(#[from] aho_corasick::MatchError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + Send>),
}

pub fn uwuify<S: AsRef<str>>(input: S) -> Result<String, UwuError> {
    let uwu = Uwu::new();
    uwu.uwuify(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn assert_uwuify() {
        fn convert(input: &str) -> String {
            Uwu::new().uwuify(input).unwrap()
        }

        assert_eq!(convert("very nice"), "vewy nyice");
        assert_eq!(convert("very elegant solution"), "vewy ewegant s-sowution");
        assert_eq!(convert("master! suki!"), "mastew! ^^ suki! ʘwʘ");
        assert_eq!(
            convert("The quick brown fox jumps over the lazy dog"),
            "the quick b-bwown fox j-jumps ovew the wazy dog"
        );
    }

    #[test]
    fn assert_rng() {
        fn calc_avg<F>(mut func: F, rounds: usize) -> f64
        where
            F: FnMut(&mut fastrand::Rng) -> bool,
        {
            let mut rng = fastrand::Rng::with_seed(75777521); // 'uwu!' = 75 77 75 21
            let mut positives = 0;
            for _ in 0..rounds {
                if func(&mut rng) {
                    positives += 1;
                }
            }
            positives as f64 / rounds as f64
        }

        // Validate that bool() is ~50%
        assert_eq!(calc_avg(|rng| rng.bool(), 1000), 0.499); // ~0.5

        // Validate that u8(0..1) is 100%
        assert_eq!(calc_avg(|rng| rng.u8(0..1) == 0, 1000), 1.0);

        // Validate that u8(0..2) is 50%
        assert_eq!(calc_avg(|rng| rng.u8(0..2) == 0, 1000), 0.492); // ~0.5

        // Validate that u8(0..3) is 33%
        assert_eq!(calc_avg(|rng| rng.u8(0..3) == 0, 1000), 0.319); // ~0.33(3)

        // Validate that u8(0..4) is 25%
        assert_eq!(calc_avg(|rng| rng.u8(0..4) == 0, 1000), 0.231); // ~0.25

        // Validate that u8(0..5) is 20%
        assert_eq!(calc_avg(|rng| rng.u8(0..5) == 0, 1000), 0.186); // ~0.2
    }
}
