use nom::{
    bytes::complete as bytes,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    common,
    literal::Literal,
};

// =============================================================================
// Parse
// =============================================================================

// Parsers

pub fn literal(input: &str) -> IResult<&str, Literal> {
    multi::many1(
        bytes::take_while1(is_literal)
            .recognize()
            .or(common::percent_encoded),
    )
    .map(|output| output.concat())
    .map(Literal)
    .parse(input)
}

// -----------------------------------------------------------------------------

// Predicates

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

    // Literal

    #[test]
    fn literal_ok() {
        [
            ("valid", "", "valid"),
            ("valid invalid", " invalid", "valid"),
            ("valid%2b invalid", " invalid", "valid%2b"),
            ("valid%2k invalid", "%2k invalid", "valid"),
            ("%2bvalid invalid", " invalid", "%2bvalid"),
            ("'", "", "'"),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, ok))| {
            assert_eq!(
                literal(input),
                Ok((rest, Literal::new(ok))),
                "Test Case {i}"
            );
        });
    }

    #[test]
    fn literal_err() {
        [
            (" invalid", " invalid", ErrorKind::Char),
            ("|invalid", "|invalid", ErrorKind::Char),
            ("%2ketc", "2ketc", ErrorKind::TakeWhileMN),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, kind))| {
            assert_eq!(
                literal(input),
                Err(Err::Error(Error::new(rest, kind))),
                "Test Case {i}"
            );
        });
    }
}
