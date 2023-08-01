use nom::{
    bytes::complete as bytes,
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

// =============================================================================
// Explode
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct Prefix(pub usize);

// -----------------------------------------------------------------------------

// Parsing

impl Prefix {
    pub fn parse(input: &str) -> IResult<&str, Prefix> {
        character::satisfy(is_non_zero_digit)
            .and::<_, &str>(bytes::take_while_m_n(0, 3, is_digit))
            .map(|(digit, digits)| {
                let mut src = String::with_capacity(digits.len() + 1);
                src.push(digit);
                src.push_str(digits);
                src.parse::<u16>().unwrap().into()
            })
            .preceded_by(character::char(':'))
            .map(Prefix)
            .parse(input)
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_digit(c: char) -> bool {
    match c {
        _ if c.is_ascii_digit() => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_non_zero_digit(c: char) -> bool {
    match c {
        | '\x31'..='\x39' => true,
        _ => false,
    }
}
