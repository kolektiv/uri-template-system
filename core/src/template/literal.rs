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
    ParseRef,
};

// =================================================s============================
// Literal
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'a> {
    parse_ref: ParseRef<'a>,
}

impl<'a> Literal<'a> {
    const fn new(parse_ref: ParseRef<'a>) -> Self {
        Self { parse_ref }
    }
}

impl<'a> Parse<'a> for Literal<'a> {
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
        let mut state = State::default();

        for (i, c) in raw.char_indices() {
            match &state.next {
                // TODO: Experiment with ordering here - may or may not have perf impact
                Next::Literal if is_literal(c) => continue,
                Next::Literal if is_percent(c) => state.next = Next::Hex1,
                Next::Literal if i > 0 => {
                    let len = i;
                    let parse_ref = ParseRef::new(base, base + i - 1, &raw[..i]);

                    return Ok((len, Self::new(parse_ref)));
                }
                Next::Hex1 if is_hex_digit(c) => state.next = Next::Hex2,
                Next::Hex2 if is_hex_digit(c) => state.next = Next::Literal,
                _ => {
                    return Err(Error::msg("lit: expected valid char(s)"));
                }
            }
        }

        Ok((
            raw.len(),
            Self::new(ParseRef::new(base, base + raw.len() - 1, raw)),
        ))
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
        | '\u{000000}'..='\u{000020}'           // ASCII Ctl     | ASCII Range
        | '\u{000022}'                          // ASCII Misc    | ASCII Range
        | '\u{000025}'                          //               | ASCII Range
        | '\u{00003c}'                          //               | ASCII Range
        | '\u{00003e}'                          //               | ASCII Range
        | '\u{00005c}'                          //               | ASCII Range
        | '\u{00005e}'                          //               | ASCII Range
        | '\u{000060}'                          //               | ASCII Range
        | '\u{00007b}'..='\u{00007d}'           //               | ASCII Range
        | '\u{00007f}'..='\u{00009f}' => false, // Unicode Ctl   | Unicode Range
        _ => true,
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
        output.push_str_encode(self.parse_ref.slice, common::reserved());
    }
}
