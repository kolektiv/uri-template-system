use anyhow::Result;

use crate::{
    template::component::expression::variable_specification::VarSpec,
    TryParse,
};

// =============================================================================
// Variable List
// =============================================================================

// Types

pub type VariableList<'t> = Vec<VarSpec<'t>>;

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for VariableList<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut parsed_varspecs = Self::new();
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Comma if raw[state.position..].starts_with(',') => {
                    state.next = Next::VarSpec;
                    state.position += 1;
                }
                Next::Comma => return Ok((state.position, parsed_varspecs)),
                Next::VarSpec => match VarSpec::try_parse(&raw[state.position..]) {
                    Ok((position, varspec)) => {
                        parsed_varspecs.push(varspec);

                        state.next = Next::Comma;
                        state.position += position;
                    }
                    Err(err) => return Err(err),
                },
            }
        }
    }
}

#[derive(Default)]
struct State {
    next: Next,
    position: usize,
}

#[derive(Default)]
enum Next {
    Comma,
    #[default]
    VarSpec,
}
