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
