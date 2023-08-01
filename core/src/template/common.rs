use std::sync::OnceLock;

use nom::{
    bytes::complete as bytes,
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::codec::Encoding;

// =============================================================================
// Common
// =============================================================================

// Parsing

pub fn percent_encoded(input: &str) -> IResult<&str, &str> {
    character::char('%')
        .and(bytes::take_while_m_n(2, 2, is_hex_digit))
        .recognize()
        .parse(input)
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_hex_digit(c: char) -> bool {
    match c {
        _ if c.is_ascii_hexdigit() => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Expansion

static RESERVED: OnceLock<Encoding> = OnceLock::new();
static UNRESERVED: OnceLock<Encoding> = OnceLock::new();

pub fn reserved() -> &'static Encoding {
    RESERVED.get_or_init(|| Encoding {
        allow_encoded: true,
        allow: Box::new(|c| is_unreserved(c) || is_reserved(c)),
    })
}

pub fn unreserved() -> &'static Encoding {
    UNRESERVED.get_or_init(|| Encoding {
        allow_encoded: false,
        allow: Box::new(is_unreserved),
    })
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_gen_delim(c: char) -> bool {
    match c {
        | '\x23'
        | '\x2f'
        | '\x3a'
        | '\x3f'
        | '\x40'
        | '\x5b'
        | '\x5d' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_sub_delim(c: char) -> bool {
    match c {
        | '\x21'
        | '\x24'
        | '\x26'..='\x2c'
        | '\x3b'
        | '\x3d' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_reserved(c: char) -> bool {
    match c {
        _ if is_gen_delim(c) => true,
        _ if is_sub_delim(c) => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_unreserved(c: char) -> bool {
    match c {
        | '\x30'..='\x39'
        | '\x41'..='\x5a'
        | '\x61'..='\x7a'
        | '\x2d'..='\x2e'
        | '\x5f'
        | '\x7e' => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use nom::{
        error::{
            Error,
            ErrorKind,
        },
        Err,
    };

    use super::*;

    #[test]
    fn percent_encoded_ok() {
        [
            ("%2b", "", "%2b"),
            ("%2B", "", "%2B"),
            ("%2b rest", " rest", "%2b"),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, output))| {
            assert_eq!(percent_encoded(input), Ok((rest, output)), "Test Case {i}");
        })
    }

    #[test]
    fn percent_encoded_err() {
        [
            ("$2b", "$2b", ErrorKind::Char),
            ("%2g", "2g", ErrorKind::TakeWhileMN),
            ("%ge", "ge", ErrorKind::TakeWhileMN),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, kind))| {
            assert_eq!(
                percent_encoded(input),
                Err(Err::Error(Error::new(rest, kind))),
                "Test Case {i}"
            );
        })
    }
}
