use nom::{
    bytes::complete as bytes,
    character::complete as character,
    multi,
    AsChar,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::util::{self,};

// =============================================================================
// Literal
// =============================================================================

// Type

#[derive(Debug, PartialEq)]
pub struct Literal(String);

impl Literal {
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
            .or(character::char('%')
                .and(bytes::take_while_m_n(2, 2, AsChar::is_hex_digit))
                .recognize()),
    )
    .map(|output| output.concat())
    .map(Literal)
    .parse(input)
}

// -----------------------------------------------------------------------------

// Predicates

fn is_ucschar_value(v: u32) -> bool {
    util::is_in_range(v, 0xa0, 0xd7ff)
        || util::is_in_range(v, 0xf900, 0xfdcf)
        || util::is_in_range(v, 0xfdf0, 0xffef)
        || util::is_in_range(v, 0x10000, 0x1fffd)
        || util::is_in_range(v, 0x20000, 0x2fffd)
        || util::is_in_range(v, 0x30000, 0x3fffd)
        || util::is_in_range(v, 0x40000, 0x4fffd)
        || util::is_in_range(v, 0x50000, 0x5fffd)
        || util::is_in_range(v, 0x60000, 0x6fffd)
        || util::is_in_range(v, 0x70000, 0x7fffd)
        || util::is_in_range(v, 0x80000, 0x8fffd)
        || util::is_in_range(v, 0x90000, 0x9fffd)
        || util::is_in_range(v, 0xa0000, 0xafffd)
        || util::is_in_range(v, 0xb0000, 0xbfffd)
        || util::is_in_range(v, 0xc0000, 0xcfffd)
        || util::is_in_range(v, 0xd0000, 0xdfffd)
        || util::is_in_range(v, 0xe0000, 0xefffd)
}

fn is_iprivate_value(v: u32) -> bool {
    util::is_in_range(v, 0xe000, 0xf8ff)
        || util::is_in_range(v, 0xf0000, 0xffffd)
        || util::is_in_range(v, 0x100000, 0x10fffd)
}

fn is_literal_value(v: u32) -> bool {
    util::is_equal_to(v, 0x21)
        || util::is_in_range(v, 0x23, 0x24)
        || util::is_equal_to(v, 0x26)
        || util::is_in_range(v, 0x28, 0x3b)
        || util::is_equal_to(v, 0x3d)
        || util::is_in_range(v, 0x3f, 0x5b)
        || util::is_equal_to(v, 0x5d)
        || util::is_equal_to(v, 0x5f)
        || util::is_in_range(v, 0x61, 0x7a)
        || util::is_equal_to(v, 0x7e)
}

fn is_literal(value: u32) -> bool {
    is_literal_value(value) || is_ucschar_value(value) || is_iprivate_value(value)
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        [
            ("valid", ("", "valid")),
            ("valid invalid", (" invalid", "valid")),
            ("valid%2b invalid", (" invalid", "valid%2b")),
            ("valid%2k invalid", ("%2k invalid", "valid")),
            ("%2bvalid invalid", (" invalid", "%2bvalid")),
        ]
        .into_iter()
        .for_each(|(input, (rest, result))| {
            assert_eq!(Literal::parse(input), Ok((rest, Literal(result.into()))))
        });
    }
}
