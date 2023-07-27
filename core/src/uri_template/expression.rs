#![allow(dead_code)]

use std::slice;

use nom::{
    bytes::complete as bytes,
    character::complete as character,
    multi,
    sequence,
    AsChar,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::uri_template::common;

// =============================================================================
// Expression
// =============================================================================

// Types

#[derive(Debug, PartialEq)]
pub struct Expression(Vec<VarSpec>, Option<Operator>);

impl Expression {
    pub fn new(variable_list: Vec<VarSpec>, operator: Option<Operator>) -> Self {
        Self(variable_list, operator)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        expression(input)
    }
}

#[derive(Debug, PartialEq)]
pub struct VarSpec(String, Option<Modifier>);

impl VarSpec {
    pub fn new<S>(varname: S, modifier: Option<Modifier>) -> Self
    where
        S: Into<String>,
    {
        Self(varname.into(), modifier)
    }
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Prefix(usize),
    Explode,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Level2(OpLevel2),
    Level3(OpLevel3),
    Reserve(OpReserve),
}

#[derive(Debug, PartialEq)]
pub enum OpLevel2 {
    Plus,
    Hash,
}

#[derive(Debug, PartialEq)]
pub enum OpLevel3 {
    Period,
    Slash,
    Semicolon,
    Question,
    Ampersand,
}

#[derive(Debug, PartialEq)]
pub enum OpReserve {
    Equals,
    Comma,
    Exclamation,
    At,
    Pipe,
}

// -----------------------------------------------------------------------------

// Parsers

// Note: varname and varchar are currently implemented as direct representations
// of the grammar as specified in RFC6570 - this is probably less than optimal
// for parsing, where parsing by, effectively, individual characters or encoded
// characters produces far more intermediate values than parsing sets of
// matching characters - however, for now this will stay, pending any future
// optimisation of this part of the library.

fn expression(input: &str) -> IResult<&str, Expression> {
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
    character::satisfy(|c| c >= '\x31' && c <= '\x39')
        .and::<_, &str>(bytes::take_while_m_n(0, 3, AsChar::is_dec_digit))
        .map(|(digit, digits)| {
            let mut src = String::with_capacity(digits.len() + 1);
            src.push(digit);
            src.push_str(digits);

            u16::from_str_radix(&src, 10)
                .expect("max length parse error")
                .into()
        })
        .preceded_by(character::char(':'))
        .map(|max_length| Modifier::Prefix(max_length))
        .parse(input)
}

fn explode(input: &str) -> IResult<&str, Modifier> {
    character::char('*').map(|_| Modifier::Explode).parse(input)
}

// -----------------------------------------------------------------------------

// Predicates

fn is_varchar(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression() {
        [
            (
                "{valid}",
                "",
                Expression::new(vec![VarSpec::new("valid", None)], None),
            ),
            // ("valid.valid", "", VarSpec::new("valid.valid", None)),
            // ("valid invalid", " invalid", VarSpec::new("valid", None)),
            // ("v_29.m-invalid", "-invalid", VarSpec::new("v_29.m", None)),
            // ("valid*", "", VarSpec::new("valid", Some(Modifier::Explode))),
            // ("va:12", "", VarSpec::new("va", Some(Modifier::Prefix(12)))),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (input, rest, ok))| {
            assert_eq!(expression(input), Ok((rest, ok)), "Test Case {i}")
        });
    }
}
