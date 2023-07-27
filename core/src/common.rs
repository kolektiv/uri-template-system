use nom::{
    bytes::complete as bytes,
    character::complete as character,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

// =============================================================================
// Common
// =============================================================================

// Parsers

pub fn percent_encoded(input: &str) -> IResult<&str, &str> {
    character::char('%')
        .and(bytes::take_while_m_n(2, 2, is_hex_digit))
        .recognize()
        .parse(input)
}

// -----------------------------------------------------------------------------

// Predicates

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_hex_digit(c: char) -> bool {
    match c {
        _ if c.is_ascii_hexdigit() => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {}
