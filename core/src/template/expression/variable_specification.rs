use std::iter::Peekable;

use anyhow::Result;

use crate::{
    template::expression::{
        modifier::Modifier,
        variable_list::VariableList,
        variable_name::VarName,
    },
    value::{
        Value,
        Values,
    },
    Parse,
};

// =============================================================================
// VarSpec
// =============================================================================

// Types

pub type VarSpec<'a> = (VarName<'a>, Option<Modifier<'a>>);

impl<'a> Parse<'a> for VarSpec<'a> {
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
        VarName::parse(raw, base).and_then(|(position_a, varname)| {
            Option::<Modifier>::parse(&raw[position_a..], base + position_a).and_then(
                |(position_b, modifier)| Ok((position_a + position_b, (varname, modifier))),
            )
        })
    }
}

// -----------------------------------------------------------------------------

// Expansion

pub fn defined<'a, 'v>(
    variable_list: &'v VariableList<'a>,
    values: &'v Values,
) -> Peekable<impl Iterator<Item = (&'v Value, &'v VarSpec<'a>)> + 'v> {
    variable_list
        .iter()
        .filter_map(|varspec| values.get(varspec.0.value()).map(|value| (value, varspec)))
        .peekable()
}
