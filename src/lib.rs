//! Convert text to an uwuified version.
//!
//! ## Quick start
//!
//! Add the uwu library to your project with:
//!
//! ```shell
//! cargo add uwu-rs
//! ```
//!
//! Then use it:
//!
//! ```rust
//! let uwuified = uwu_rs::Uwu::new().uwuify("Hello world!");
//! ```
//!
//! Or, if you want more control over the generated output:
//!
//! ```
//! let uwu = uwu_rs::Uwu::builder()
//!     .lowercase()
//!     .expressions()
//!     .w_replace()
//!     .stutter(4)
//!     .emojis(1)
//!     .build();
//! let uwuified = uwu.uwuify("Hello world!");
//! ```

#![warn(missing_docs)]

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, Input};
use std::io::Write;
use thiserror::Error;

mod builder;
mod dict;

pub use builder::*;

/// An `Uwu` instance capable of running the uwu algorithm.
///
/// The simplest way to use it is:
/// ```
/// let uwuified = uwu_rs::Uwu::new().uwuify("Hello world!");
/// ```
///
/// But it is also possible to modify its behaviour with:
/// ```
/// let uwu = uwu_rs::Uwu::builder()
///     .lowercase()
///     .expressions()
///     .w_replace()
///     .stutter(4)
///     .emojis(1)
///     .build();
/// let uwuified = uwu.uwuify("Hello world!");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Uwu {
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

impl Default for Uwu {
    fn default() -> Self {
        Self {
            lowercase: true,
            expressions: true,
            w_replace: true,
            stutter: true,
            stutter_chance: 4,
            emojis: true,
            emojis_chance: 1,
        }
    }
}

impl Uwu {
    /// Create a new instance of `Uwu` with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new instance of a builder for the `Uwu` instance.
    pub fn builder() -> UwuBuilder {
        UwuBuilder::new()
    }

    /// Converts the input text into an uwuified version.
    ///
    /// Example:
    /// ```
    /// let uwuified = uwu_rs::Uwu::new().uwuify("Hello world!");
    /// ```
    pub fn uwuify<S: AsRef<str>>(&self, input: S) -> Result<String, UwuError> {
        let mut input = input.as_ref().to_owned();

        if self.lowercase {
            input = input.to_ascii_lowercase();
        }

        let mut buf = input.into_bytes();
        // Pad input with spaces so features that only operate after spaces work on the first and
        // last word.
        buf.insert(0, b' ');
        buf.push(b' ');

        if self.expressions {
            buf = Self::do_expressions(buf)?;
        }
        if self.w_replace {
            buf = Self::do_w_replace(buf)?;
        }
        if self.stutter {
            buf = self.do_stutter(buf)?;
        }
        if self.emojis {
            buf = self.do_emojis(buf)?;
        }

        // Remove the padding added in the start
        if let Some(last) = buf.last() {
            if *last == b' ' {
                buf.pop();
            }
        }
        if let Some(first) = buf.first() {
            if *first == b' ' {
                buf.remove(0);
            }
        }

        let output = String::from_utf8_lossy(&buf).to_string();
        Ok(output)
    }

    fn do_expressions(input: Vec<u8>) -> Result<Vec<u8>, UwuError> {
        let mut buf = Vec::with_capacity(input.len());
        let matcher = AhoCorasick::new(dict::EXPRESSIONS)?;
        matcher.try_stream_replace_all(
            input.as_slice(),
            &mut buf,
            dict::EXPRESSIONS_REPLACE.as_slice(),
        )?;
        Ok(buf)
    }

    fn do_w_replace(mut input: Vec<u8>) -> Result<Vec<u8>, UwuError> {
        input.iter_mut().for_each(|byte| {
            if matches!(byte, b'l' | b'r') {
                *byte = b'w';
            }
        });
        Ok(input)
    }

    fn do_stutter(&self, input: Vec<u8>) -> Result<Vec<u8>, UwuError> {
        if input.len() < 2 {
            return Ok(input);
        }

        let mut buf: Vec<u8> = Vec::with_capacity(input.len());
        let mut rng = Uwu::create_rng();

        let mut prev_idx = 0;
        for mut idx in 0..input.len() - 1 {
            if input[idx] == b' '
                && input[idx + 1].is_ascii_alphabetic()
                && rng.u8(0..self.stutter_chance) == 0
            {
                idx += 1;
                let section = &input[prev_idx..idx];
                let ch = input[idx];
                buf.write_all(section)?;
                buf.write_all(&[ch])?;
                buf.write_all(&[b'-'])?;
                prev_idx = idx;
            }
        }

        // Dump remaining
        buf.write_all(&input[prev_idx..])?;

        Ok(buf)
    }

    fn do_emojis(&self, input: Vec<u8>) -> Result<Vec<u8>, UwuError> {
        let matcher = AhoCorasickBuilder::new().build(dict::PUNCTUATION)?;
        let matches = matcher
            .try_find_iter(Input::new(&input))?
            .map(|mat| mat.end())
            .collect::<Vec<usize>>();
        if matches.is_empty() {
            return Ok(input);
        }

        let mut buf = Vec::with_capacity(input.len());
        let mut rng = Uwu::create_rng();

        let mut prev_idx = 0;
        for idx in matches {
            if rng.u8(0..self.emojis_chance) != 0 {
                continue;
            }
            let section = &input[prev_idx..idx];
            let emoji = rng.choice(dict::EMOJIS).unwrap_or("uwu");
            buf.write_all(section)?;
            buf.write_all(emoji.as_bytes())?;
            prev_idx = idx;
        }

        // Dump remaining
        buf.write_all(&input[prev_idx..])?;

        Ok(buf)
    }

    fn create_rng() -> fastrand::Rng {
        let seed = 75777521; // 'uwu!' = 75 77 75 21
        fastrand::Rng::with_seed(seed)
    }
}

/// A Uwu error.
#[derive(Error, Debug)]
pub enum UwuError {
    /// Error building string matcher
    #[error("string matcher build error: {0}")]
    StringMatcherBuild(#[from] aho_corasick::BuildError),
    /// Error in string match
    #[error("string matcher match error: {0}")]
    StringMatcherMatch(#[from] aho_corasick::MatchError),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Unknown error
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + Send>),
}

/// Converts text to an uwuified version. This is a utility method for using
/// `Uwu::new().uwuify("input")`.
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
        assert_eq!(convert("master! suki!"), "mastew! o.O suki! ^•ﻌ•^");
        assert_eq!(
            convert("my master, I have a question"),
            "my mastew, o.O i-i have a-a qwuestion"
        );
        assert_eq!(
            convert("The quick brown fox jumps over the lazy dog"),
            "the qwuick b-bwown fox j-jumps ovew the wazy dog"
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
