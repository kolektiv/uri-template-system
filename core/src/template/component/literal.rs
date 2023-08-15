use std::fmt::{
    self,
    Formatter,
};

use anyhow::{
    Error,
    Result,
};

use crate::{
    common::matcher::{
        Ascii,
        Matcher,
        PercentEncoded,
        Unicode,
    },
    expansion::Expand,
    template::component::{
        self,
        EncodeExt,
    },
    value::Values,
    TryParse,
};

// =================================================s============================
// Literal
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'t> {
    raw: &'t str,
}

impl<'t> Literal<'t> {
    const fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

impl<'t> TryParse<'t> for Literal<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        match parse_matcher().matches(raw) {
            0 => Err(Error::msg("lit: expected valid char(s)")),
            n => Ok((n, Literal::new(&raw[..n]))),
        }
    }
}

const fn parse_matcher() -> impl Matcher {
    (
        Ascii::new(is_literal_ascii),
        PercentEncoded,
        Unicode::new(is_literal_unicode),
    )
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
fn is_literal_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x3f'..=b'\x5b' // ?, @, A..Z, [
        | b'\x26'..=b'\x3b' // &, ', (, ),*, +, ,, -, -, ., /, 0..9, :, ;,
        | b'\x21'           // !
        | b'\x23'..=b'\x24' // #, $
        | b'\x3d'           // =
        | b'\x5d'           // ]
        | b'\x5f'           // _
        | b'\x7e' => true,  // ~
        _ => false
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
fn is_literal_unicode(c: char) -> bool {
    match c {
        | '\u{0000a0}'..='\u{00d7ff}' // ucschar...
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
        | '\u{0e0000}'..='\u{0efffd}'
        | '\u{00e000}'..='\u{00f8ff}' // iprivate...
        | '\u{0f0000}'..='\u{0ffffd}'
        | '\u{100000}'..='\u{10fffd}' => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl<'t> Expand for Literal<'t> {
    fn expand(&self, _values: &Values, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str_encoded(self.raw, &encode_matcher())
    }
}

pub const fn encode_matcher() -> impl Matcher {
    (
        Ascii::new(|b| component::is_unreserved_ascii(b) || component::is_reserved_ascii(b)),
        PercentEncoded,
    )
}
