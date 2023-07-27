use std::slice;

use nom::{
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
        Modifier,
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

pub fn expression(input: &str) -> IResult<&str, Expression> {
    sequence::delimited(character::char('{'), variable_list, character::char('}'))
        .map(|variable_list| Expression::new(variable_list, None))
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
    use super::*;

    #[test]
    fn parse_expression() {
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
            )
        });
    }
}
