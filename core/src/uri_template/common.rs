use nom::{
    bytes::complete as bytes,
    character::complete as character,
    AsChar,
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
        .and(bytes::take_while_m_n(2, 2, AsChar::is_hex_digit))
        .recognize()
        .parse(input)
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {}
