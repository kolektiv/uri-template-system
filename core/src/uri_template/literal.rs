use nom::{
    bytes::complete as bytes,
    character::complete as character,
    multi,
    AsChar,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

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
    .map(concat)
    .map(Literal)
    .parse(input)
}

// -----------------------------------------------------------------------------

// Literal Predicates

fn is_ucschar_value(v: u32) -> bool {
    is_in_range(v, 0xa0, 0xd7ff)
        || is_in_range(v, 0xf900, 0xfdcf)
        || is_in_range(v, 0xfdf0, 0xffef)
        || is_in_range(v, 0x10000, 0x1fffd)
        || is_in_range(v, 0x20000, 0x2fffd)
        || is_in_range(v, 0x30000, 0x3fffd)
        || is_in_range(v, 0x40000, 0x4fffd)
        || is_in_range(v, 0x50000, 0x5fffd)
        || is_in_range(v, 0x60000, 0x6fffd)
        || is_in_range(v, 0x70000, 0x7fffd)
        || is_in_range(v, 0x80000, 0x8fffd)
        || is_in_range(v, 0x90000, 0x9fffd)
        || is_in_range(v, 0xa0000, 0xafffd)
        || is_in_range(v, 0xb0000, 0xbfffd)
        || is_in_range(v, 0xc0000, 0xcfffd)
        || is_in_range(v, 0xd0000, 0xdfffd)
        || is_in_range(v, 0xe0000, 0xefffd)
}

fn is_iprivate_value(v: u32) -> bool {
    is_in_range(v, 0xe000, 0xf8ff)
        || is_in_range(v, 0xf0000, 0xffffd)
        || is_in_range(v, 0x100000, 0x10fffd)
}

fn is_literal_value(v: u32) -> bool {
    is_equal_to(v, 0x21)
        || is_in_range(v, 0x23, 0x24)
        || is_equal_to(v, 0x26)
        || is_in_range(v, 0x28, 0x3b)
        || is_equal_to(v, 0x3d)
        || is_in_range(v, 0x3f, 0x5b)
        || is_equal_to(v, 0x5d)
        || is_equal_to(v, 0x5f)
        || is_in_range(v, 0x61, 0x7a)
        || is_equal_to(v, 0x7e)
}

fn is_literal(value: u32) -> bool {
    is_literal_value(value) || is_ucschar_value(value) || is_iprivate_value(value)
}

// -----------------------------------------------------------------------------

// Generic Predicates

fn is_equal_to(a: u32, b: u32) -> bool {
    a == b
}

fn is_in_range(a: u32, min: u32, max: u32) -> bool {
    a >= min && a <= max
}

// -----------------------------------------------------------------------------

// Generic Utilities

fn concat(inputs: Vec<&str>) -> String {
    fn init(inputs: &Vec<&str>) -> String {
        String::with_capacity(inputs.iter().map(|i| i.len()).sum())
    }

    inputs.iter().fold(init(&inputs), |mut output, input| {
        output.push_str(input);
        output
    })
}

// -----------------------------------------------------------------------------

// Tests

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Literal::parse("valid"), Ok(("", Literal("valid".into()))))
    }
}
