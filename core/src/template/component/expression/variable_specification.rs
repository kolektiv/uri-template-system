use anyhow::Result;

use crate::{
    template::component::expression::{
        modifier::Modifier,
        variable_name::VarName,
    },
    TryParse,
};

// =============================================================================
// Variable Specification
// =============================================================================

// Types

pub type VarSpec<'t> = (VarName<'t>, Option<Modifier<'t>>);

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for VarSpec<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        VarName::try_parse(raw).and_then(|(position_a, varname)| {
            Option::<Modifier>::try_parse(&raw[position_a..]).and_then(|(position_b, modifier)| {
                Ok((position_a + position_b, (varname, modifier)))
            })
        })
    }
}
