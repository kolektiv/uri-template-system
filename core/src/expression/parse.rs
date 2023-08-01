use std::slice;

use nom::{
    branch,
    bytes::complete as bytes,
    character::complete as character,
    multi,
    sequence,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    common,
    expression::{
        Expression,
        Fragment,
        Modifier,
        OpLevel2,
        OpLevel3,
        OpReserve,
        Operator,
        Reserved,
        VarSpec,
    },
};

// =============================================================================
// Parse
// =============================================================================

// Parsers

// Note: varname and varchar are currently implemented as direct representations
// of the grammar as specified in RFC6570 - this is probably less than optimal
// for parsing, where parsing by, effectively, individual characters or encoded
// characters produces far more intermediate values than parsing sets of
// matching characters - however, for now this will stay, pending any future
// optimisation of this part of the library.

// Also note: parsers are written to meet the specifications individually, and
// do not provide guarantees about anterior or posterior input state. For
// example, the prefix modifier will restrict the input to 4 digits in length,
// but will not guarantee that it is not followed by additional digits - this is
// assumed to be an issue for a subsequent parser if it does not accept the
// remaining input.

pub fn expression(input: &str) -> IResult<&str, Expression> {
    sequence::delimited(
        character::char('{'),
        operator.opt().and(variable_list),
        character::char('}'),
    )
    .map(|(operator, variable_list)| Expression::new(variable_list, operator))
    .parse(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    branch::alt((
        character::char('+').value(Operator::Level2(OpLevel2::Reserved(Reserved))),
        character::char('#').value(Operator::Level2(OpLevel2::Fragment(Fragment))),
        character::char('.').value(Operator::Level3(OpLevel3::Label)),
        character::char('/').value(Operator::Level3(OpLevel3::Path)),
        character::char(';').value(Operator::Level3(OpLevel3::PathParameter)),
        character::char('?').value(Operator::Level3(OpLevel3::Query)),
        character::char('&').value(Operator::Level3(OpLevel3::QueryContinuation)),
        character::char('=').value(Operator::Reserve(OpReserve::Equals)),
        character::char(',').value(Operator::Reserve(OpReserve::Comma)),
        character::char('!').value(Operator::Reserve(OpReserve::Exclamation)),
        character::char('@').value(Operator::Reserve(OpReserve::At)),
        character::char('|').value(Operator::Reserve(OpReserve::Pipe)),
    ))
    .parse(input)
}

fn variable_list(input: &str) -> IResult<&str, Vec<VarSpec>> {
    multi::separated_list1(character::char(','), varspec).parse(input)
}

fn varspec(input: &str) -> IResult<&str, VarSpec> {
    varname
        .and(modifier.opt())
        .map(|(varname, modifier)| VarSpec(varname, modifier))
        .parse(input)
}

fn varname(input: &str) -> IResult<&str, String> {
    varchar
        .and(
            multi::many0(
                character::char('.')
                    .opt()
                    .recognize()
                    .and(varchar)
                    .map(|(dot, varchar)| Vec::from_iter([dot, varchar])),
            )
            .map(|output| output.concat()),
        )
        .map(|(output_a, output_b)| [slice::from_ref(&output_a), &output_b].concat())
        .map(|output| output.concat())
        .parse(input)
}

fn varchar(input: &str) -> IResult<&str, &str> {
    character::satisfy(is_varchar)
        .recognize()
        .or(common::percent_encoded)
        .parse(input)
}

fn modifier(input: &str) -> IResult<&str, Modifier> {
    prefix.or(explode).parse(input)
}

fn prefix(input: &str) -> IResult<&str, Modifier> {
    character::satisfy(is_non_zero_digit)
        .and::<_, &str>(bytes::take_while_m_n(0, 3, is_digit))
        .map(|(digit, digits)| {
            let mut src = String::with_capacity(digits.len() + 1);
            src.push(digit);
            src.push_str(digits);
            src.parse::<u16>().unwrap().into()
        })
        .preceded_by(character::char(':'))
        .map(Modifier::Prefix)
        .parse(input)
}

fn explode(input: &str) -> IResult<&str, Modifier> {
    character::char('*').map(|_| Modifier::Explode).parse(input)
}

// -----------------------------------------------------------------------------

// Predicates

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

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_varchar(c: char) -> bool {
    match c {
        | '\x5f' => true,
        _ if c.is_ascii_alphanumeric() => true,
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
    fn varname_ok() {
        [
            ("a", "", "a"),
            ("a.", ".", "a"),
            ("3", "", "3"),
            ("ab", "", "ab"),
            ("a_b", "", "a_b"),
            ("%2b", "", "%2b"),
            ("a_b.c", "", "a_b.c"),
            ("a%2b.c", "", "a%2b.c"),
            ("a%2b.c!", "!", "a%2b.c"),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, result))| {
            assert_eq!(varname(input), Ok((rest, result.into())), "Test Case {i}");
        });
    }

    #[test]
    fn varname_err() {
        [
            (".", ".", ErrorKind::Char),
            (".a", ".a", ErrorKind::Char),
            ("$2b", "$2b", ErrorKind::Char),
            ("/", "/", ErrorKind::Char),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, kind))| {
            assert_eq!(
                varname(input),
                Err(Err::Error(Error::new(rest, kind))),
                "Test Case {i}"
            );
        });
    }

    // -------------------------------------------------------------------------

    // Varchar

    #[test]
    fn varchar_ok() {
        [
            ("a", "", "a"),
            ("3", "", "3"),
            ("ab", "b", "a"),
            ("_", "", "_"),
            ("%2b", "", "%2b"),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, result))| {
            assert_eq!(varchar(input), Ok((rest, result)), "Test Case {i}");
        });
    }

    #[test]
    fn varchar_err() {
        [
            (".", ".", ErrorKind::Char),
            ("$2b", "$2b", ErrorKind::Char),
            ("/", "/", ErrorKind::Char),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, kind))| {
            assert_eq!(
                varchar(input),
                Err(Err::Error(Error::new(rest, kind))),
                "Test Case {i}"
            );
        });
    }

    // -------------------------------------------------------------------------

    // Modifier

    #[test]
    fn modifier_ok() {
        [
            ("*", "", Modifier::Explode),
            (":42", "", Modifier::Prefix(42)),
            (":10000", "0", Modifier::Prefix(1000)),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, result))| {
            assert_eq!(modifier(input), Ok((rest, result)), "Test Case {i}");
        });
    }

    #[test]
    fn modifier_err() {
        [
            (":042", ":042", ErrorKind::Char),
            (":x42", ":x42", ErrorKind::Char),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, kind))| {
            assert_eq!(
                modifier(input),
                Err(Err::Error(Error::new(rest, kind))),
                "Test Case {i}"
            );
        });
    }

    // -------------------------------------------------------------------------

    // Expression

    #[test]
    fn expression_ok() {
        [
            ("{valid}", "", vec![("valid", None)], None),
            // ("valid.valid", "", VarSpec::new("valid.valid", None)),
            // ("valid invalid", " invalid", VarSpec::new("valid", None)),
            // ("v_29.m-invalid", "-invalid", VarSpec::new("v_29.m", None)),
            // ("valid*", "", VarSpec::new("valid", Some(Modifier::Explode))),
            // ("va:12", "", VarSpec::new("va", Some(Modifier::Prefix(12)))),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, varspecs, operator))| {
            assert_eq!(
                expression(input),
                Ok((
                    rest,
                    Expression::new(
                        varspecs
                            .into_iter()
                            .map(|(name, modifier)| VarSpec::new(name, modifier))
                            .collect(),
                        operator
                    )
                )),
                "Test Case {i}"
            );
        });
    }
}
