use anyhow::Result;

use crate::{
    template::expression::variable_specification::VarSpec,
    Parse,
};

pub type VariableList<'a> = Vec<VarSpec<'a>>;

impl<'a> Parse<'a> for VariableList<'a> {
    fn parse(raw: &'a str) -> Result<(usize, Self)> {
        let mut parsed_varspecs = Self::new();
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Comma if raw[state.position..].starts_with(',') => {
                    state.next = Next::VarSpec;
                    state.position += 1;
                }
                Next::Comma => return Ok((state.position, parsed_varspecs)),
                Next::VarSpec => match VarSpec::parse(&raw[state.position..]) {
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
