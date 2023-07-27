use nom::{
    bytes::complete as bytes,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::uri_template::common;

// =============================================================================
// Literal
// =============================================================================

// Type

#[derive(Debug, PartialEq)]
pub struct Literal(String);

impl Literal {
    pub fn new<S>(literal: S) -> Self
    where
        S: Into<String>,
    {
        Self(literal.into())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        literal(input)
    }
}

// -----------------------------------------------------------------------------

// Parsers

fn literal(input: &str) -> IResult<&str, Literal> {
    multi::many1(
        bytes::take_while1(|c| is_literal(c as u32))
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

fn is_ucschar_value(v: u32) -> bool {
    in_range!(v, 0xa0, 0xd7ff)
        || in_range!(v, 0xf900, 0xfdcf)
        || in_range!(v, 0xfdf0, 0xffef)
        || in_range!(v, 0x10000, 0x1fffd)
        || in_range!(v, 0x20000, 0x2fffd)
        || in_range!(v, 0x30000, 0x3fffd)
        || in_range!(v, 0x40000, 0x4fffd)
        || in_range!(v, 0x50000, 0x5fffd)
        || in_range!(v, 0x60000, 0x6fffd)
        || in_range!(v, 0x70000, 0x7fffd)
        || in_range!(v, 0x80000, 0x8fffd)
        || in_range!(v, 0x90000, 0x9fffd)
        || in_range!(v, 0xa0000, 0xafffd)
        || in_range!(v, 0xb0000, 0xbfffd)
        || in_range!(v, 0xc0000, 0xcfffd)
        || in_range!(v, 0xd0000, 0xdfffd)
        || in_range!(v, 0xe0000, 0xefffd)
}

fn is_iprivate_value(v: u32) -> bool {
    in_range!(v, 0xe000, 0xf8ff)
        || in_range!(v, 0xf0000, 0xffffd)
        || in_range!(v, 0x100000, 0x10fffd)
}

fn is_literal_value(v: u32) -> bool {
    equal_to!(v, 0x21)
        || in_range!(v, 0x23, 0x24)
        || equal_to!(v, 0x26)
        || in_range!(v, 0x28, 0x3b)
        || equal_to!(v, 0x3d)
        || in_range!(v, 0x3f, 0x5b)
        || equal_to!(v, 0x5d)
        || equal_to!(v, 0x5f)
        || in_range!(v, 0x61, 0x7a)
        || equal_to!(v, 0x7e)
}

fn is_literal(value: u32) -> bool {
    is_literal_value(value) || is_ucschar_value(value) || is_iprivate_value(value)
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
