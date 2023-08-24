use std::fmt::Write;

use crate::{
    string::{
        satisfy::{
            self,
            Ascii,
            PercentEncoded,
            Unicode,
        },
        Encode,
        Satisfy,
    },
    template::{
        Expand,
        ExpandError,
        ParseError,
        TryParse,
    },
    value::Values,
};

// =============================================================================
// Literal
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Literal<'t> {
    value: &'t str,
}

impl<'t> Literal<'t> {
    pub const fn new(value: &'t str) -> Self {
        Self { value }
    }
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Literal<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        match satisfier().satisfy(raw) {
            0 => Err(ParseError::UnexpectedInput {
                position: global,
                message: "unexpected input parsing literal component".into(),
                expected: "valid literal characters (see: https://datatracker.ietf.org/doc/html/rfc6570#section-2.1)".into(),
            }),
            n => Ok((n, Literal::new(&raw[..n]))),
        }
    }
}

const fn satisfier() -> impl Satisfy {
    (
        Ascii::new(is_literal_ascii),
        PercentEncoded,
        Unicode::new(is_literal_unicode),
    )
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_literal_ascii(b: u8) -> bool {
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
const fn is_literal_unicode(c: char) -> bool {
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

// Expand

impl<'t> Expand for Literal<'t> {
    fn expand(&self, _values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        write.encode(self.value, &satisfy::unreserved_or_reserved())?;

        Ok(())
    }
}
