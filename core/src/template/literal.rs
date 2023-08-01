use nom::{
    bytes::complete as bytes,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    template::common,
    value::Values,
    Expand,
};

// =================================================s============================
// Literal
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Literal(String);

impl Literal {
    #[allow(dead_code)]
    fn new(literal: impl Into<String>) -> Self {
        Self(literal.into())
    }
}

// -----------------------------------------------------------------------------

// Parsing

impl Literal {
    pub fn parse(input: &str) -> IResult<&str, Literal> {
        multi::many1(
            bytes::take_while1(is_literal)
                .recognize()
                .or(common::percent_encoded),
        )
        .map(|output| output.concat())
        .map(Literal)
        .parse(input)
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_ucschar(c: char) -> bool {
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
fn is_iprivate(c: char) -> bool {
    match c {
        | '\u{00e000}'..='\u{00f8ff}'
        | '\u{0f0000}'..='\u{0ffffd}'
        | '\u{100000}'..='\u{10fffd}' => true,
        _ => false,
    }
}

// Note: The `is_literal` fn does not match the original RFC 6570, but matches
// the updates made by Errata 6937 which reinstates the "'" character as an
// allowed literal character. The "official" test cases have some additional
// tests which exercise this functionality, but it is not obvious that the test
// cases do not reflect the RFC as-was!

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_literal(c: char) -> bool {
    match c {
        | '\x21'
        | '\x23'..='\x24'
        | '\x26'..='\x3b'
        | '\x3d'
        | '\x3f'..='\x5b'
        | '\x5d'
        | '\x5f'
        | '\x61'..='\x7a'
        | '\x7e' => true,
        _ if is_ucschar(c) => true,
        _ if is_iprivate(c) => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, ()> for Literal {
    // TODO: Percentage-Encoding/Validation
    fn expand(&self, output: &mut String, _value: &Values, _context: &()) {
        output.push_str(&self.0);
    }
}

// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------

// Re-Export

// pub use self::parse::literal as parse;
