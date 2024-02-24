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

const STUTTER_CHANCE: u8 = 4;
const EMOJI_CHANCE: u8 = 2;

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

pub fn uwuify(input: &str) -> Result<String, UwuError> {
    let mut rng = fastrand::Rng::with_seed(75777521); // 'uwu!' = 75 77 75 21
    let input = " ".to_string() + &input.to_ascii_lowercase() + " ";

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
                && rng.u8(0..STUTTER_CHANCE) == 0
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
            if rng.u8(0..EMOJI_CHANCE) != 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn assert_uwuify() {
        assert_eq!(uwuify("very nice").unwrap(), "vewy nyice");
        assert_eq!(
            uwuify("very elegant solution").unwrap(),
            "vewy ewegant s-sowution"
        );
        assert_eq!(uwuify("master! suki!").unwrap(), "mastew! ^^ suki! ʘwʘ");
        assert_eq!(
            uwuify("The quick brown fox jumps over the lazy dog").unwrap(),
            "the quick b-bwown fox j-jumps ovew the wazy dog"
        );
    }
}
