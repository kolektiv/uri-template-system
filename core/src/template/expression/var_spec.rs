use std::{
    iter::Peekable,
    slice,
};

use nom::{
    character::complete as character,
    multi,
    IResult,
    Parser,
};
use nom_supreme::ParserExt;

use crate::{
    template::{
        common,
        expression::modifier::Modifier,
    },
    value::{
        Value,
        Values,
    },
};

// =============================================================================
// VarSpec
// =============================================================================

// Types

#[derive(Clone, Debug, PartialEq)]
pub struct VarSpec(pub String, pub Option<Modifier>);

impl VarSpec {
    #[allow(dead_code)]
    pub fn new(var_name: impl Into<String>, modifier: Option<Modifier>) -> Self {
        Self(var_name.into(), modifier)
    }
}

// -----------------------------------------------------------------------------

// Parsing

impl VarSpec {
    pub fn parse(input: &str) -> IResult<&str, VarSpec> {
        varname
            .and(Modifier::parse.opt())
            .map(|(varname, modifier)| VarSpec(varname, modifier))
            .parse(input)
    }
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

// Expansion

pub fn defined<'a>(
    var_specs: &'a [VarSpec],
    values: &'a Values,
) -> Peekable<impl Iterator<Item = (&'a Value, &'a VarSpec)> + 'a> {
    var_specs
        .iter()
        .filter_map(|var_spec| values.get(&var_spec.0).map(|value| (value, var_spec)))
        .peekable()
}
