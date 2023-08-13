use anyhow::{
    Error,
    Result,
};

use crate::{
    codec::Encode,
    template::common,
    value::Values,
    Expand,
    Parse,
};

// =================================================s============================
// Literal
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'a> {
    raw: &'a str,
}

impl<'a> Literal<'a> {
    const fn new(raw: &'a str) -> Self {
        Self { raw }
    }
}

impl<'a> Parse<'a> for Literal<'a> {
    fn parse(raw: &'a str) -> Result<(usize, Self)> {
        let mut state = State::default();

        for (i, c) in raw.char_indices() {
            match &state.next {
                // TODO: Experiment with ordering here - may or may not have perf impact
                Next::Literal if is_literal(c) => continue,
                Next::Literal if is_percent(c) => state.next = Next::Hex1,
                Next::Literal if i > 0 => {
                    return Ok((i, Self::new(&raw[..i])));
                }
                Next::Hex1 if is_hex_digit(c) => state.next = Next::Hex2,
                Next::Hex2 if is_hex_digit(c) => state.next = Next::Literal,
                _ => {
                    println!("raw: {raw}");
                    return Err(Error::msg("lit: expected valid char(s)"));
                }
            }
        }

        Ok((raw.len(), Self::new(raw)))
    }
}

#[derive(Default)]
struct State {
    next: Next,
}

#[derive(Default)]
enum Next {
    #[default]
    Literal,
    Hex1,
    Hex2,
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
#[inline]
const fn is_literal(c: char) -> bool {
    match c {
        | '\u{000061}'..='\u{00007a}' // a..z
        | '\u{00003f}'..='\u{00005b}' // ?, @, A..Z, [
        | '\u{000026}'..='\u{00003b}' // &, ', (, ),*, +, ,, -, -, ., /, 0..9, :, ;,
        | '\u{000021}'
        | '\u{000023}'..='\u{000024}'
        | '\u{00003d}'
        | '\u{00005d}'
        | '\u{00005f}'
        | '\u{00007e}' => true,
        | _ if is_ucschar(c) => true,
        | _ if is_iprivate(c) => true,
        _ => false
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
#[inline]
const fn is_ucschar(c: char) -> bool {
    match c {
        | '\u{0000a0}'..='\u{00d7ff}'
        | '\u{00f900}'..='\u{00fdcf}'
        | '\u{00fdf0}'..='\u{00ffef}'
        | '\u{010000}'..='\u{01fffd}'
        | '\u{020000}'..='\u{02fffd}'
        | '\u{030000}'..='\u{03fffd}'
        | '\u{040000}'..='\u{04fffd}'
        | '\u{050000}'..='\u{05fffd}'
        | '\u{060000}'..='\u{06fffd}'
        | '\u{070000}'..='\u{07fffd}'
        | '\u{080000}'..='\u{08fffd}'
        | '\u{090000}'..='\u{09fffd}'
        | '\u{0a0000}'..='\u{0afffd}'
        | '\u{0b0000}'..='\u{0bfffd}'
        | '\u{0c0000}'..='\u{0cfffd}'
        | '\u{0d0000}'..='\u{0dfffd}'
        | '\u{0e0000}'..='\u{0efffd}' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
#[inline]
const fn is_iprivate(c: char) -> bool {
    match c {
        | '\u{00e000}'..='\u{00f8ff}'
        | '\u{0f0000}'..='\u{0ffffd}'
        | '\u{100000}'..='\u{10fffd}' => true,
        _ => false,
    }
}

#[inline]
const fn is_percent(c: char) -> bool {
    c == '%'
}

#[inline]
const fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, ()> for Literal<'a> {
    fn expand(&self, output: &mut String, _values: &Values, _context: &()) {
        output.push_str_encode(self.raw, common::reserved());
    }
}
