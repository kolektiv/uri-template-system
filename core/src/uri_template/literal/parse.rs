use nom::{
    bytes::complete as bytes,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::uri_template::{
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

// Macros

macro_rules! in_range {
    ($a:ident, $min:literal, $max:literal) => {
        $a >= $min && $a <= $max
    };
}

macro_rules! equal_to {
    ($a:ident, $value:literal) => {
        $a == $value
    };
}

// -----------------------------------------------------------------------------

// Predicates

fn is_ucschar(c: char) -> bool {
    in_range!(c, '\u{0000a0}', '\u{00d7ff}')
        || in_range!(c, '\u{00f900}', '\u{00fdcf}')
        || in_range!(c, '\u{00fdf0}', '\u{00ffef}')
        || in_range!(c, '\u{010000}', '\u{01fffd}')
        || in_range!(c, '\u{020000}', '\u{02fffd}')
        || in_range!(c, '\u{030000}', '\u{03fffd}')
        || in_range!(c, '\u{040000}', '\u{04fffd}')
        || in_range!(c, '\u{050000}', '\u{05fffd}')
        || in_range!(c, '\u{060000}', '\u{06fffd}')
        || in_range!(c, '\u{070000}', '\u{07fffd}')
        || in_range!(c, '\u{080000}', '\u{08fffd}')
        || in_range!(c, '\u{090000}', '\u{09fffd}')
        || in_range!(c, '\u{0a0000}', '\u{0afffd}')
        || in_range!(c, '\u{0b0000}', '\u{0bfffd}')
        || in_range!(c, '\u{0c0000}', '\u{0cfffd}')
        || in_range!(c, '\u{0d0000}', '\u{0dfffd}')
        || in_range!(c, '\u{0e0000}', '\u{0efffd}')
}

fn is_iprivate(c: char) -> bool {
    in_range!(c, '\u{00e000}', '\u{00f8ff}')
        || in_range!(c, '\u{0f0000}', '\u{0ffffd}')
        || in_range!(c, '\u{100000}', '\u{10fffd}')
}

fn is_literal(c: char) -> bool {
    equal_to!(c, '\x21')
        || in_range!(c, '\x23', '\x24')
        || equal_to!(c, '\x26')
        || in_range!(c, '\x28', '\x3b')
        || equal_to!(c, '\x3d')
        || in_range!(c, '\x3f', '\x5b')
        || equal_to!(c, '\x5d')
        || equal_to!(c, '\x5f')
        || in_range!(c, '\x61', '\x7a')
        || equal_to!(c, '\x7e')
        || is_ucschar(c)
        || is_iprivate(c)
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
    fn parse_valid() {
        [
            ("valid", "", Literal::new("valid")),
            ("valid invalid", " invalid", Literal::new("valid")),
            ("valid%2b invalid", " invalid", Literal::new("valid%2b")),
            ("valid%2k invalid", "%2k invalid", Literal::new("valid")),
            ("%2bvalid invalid", " invalid", Literal::new("%2bvalid")),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, ok))| {
            assert_eq!(Literal::parse(input), Ok((rest, ok)), "Test Case {i}")
        });
    }

    #[test]
    fn parse_invalid() {
        [
            (" invalid", Error::new(" invalid", ErrorKind::Char)),
            ("|invalid", Error::new("|invalid", ErrorKind::Char)),
            ("%2ketc", Error::new("2ketc", ErrorKind::TakeWhileMN)),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, err))| {
            assert_eq!(Literal::parse(input), Err(Err::Error(err)), "Test Case {i}")
        });
    }
}
