use std::iter::Peekable;

use nom::{
    bytes::complete as bytes,
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
            .map(|(varname, modifier)| VarSpec(varname.to_string(), modifier))
            .parse(input)
    }
}

fn varname(input: &str) -> IResult<&str, &str> {
    varchars
        .and(multi::many0(character::char('.').and(varchars)))
        .recognize()
        .parse(input)
}

// This is a slightly more optimal encoding of the ABNF rule, as it takes
// multiple matching characters at a time rather than working on a character by
// character basis. This is probably not a big deal overall, but is slightly
// simpler.

fn varchars(input: &str) -> IResult<&str, &str> {
    multi::many1(bytes::take_while1(is_varchar).or(common::percent_encoded))
        .recognize()
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
