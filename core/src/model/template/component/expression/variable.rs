use anyhow::{
    Error,
    Result,
};

use crate::{
    model::template::component::expression::modifier::Modifier,
    process::parse::TryParse,
    util::satisfy::{
        ascii::Ascii,
        percent_encoded::PercentEncoded,
        Satisfy,
    },
};

// Variable - List

pub type VariableList<'t> = Vec<VariableSpecification<'t>>;

// Variable - Specification

pub type VariableSpecification<'t> = (VariableName<'t>, Option<Modifier<'t>>);

// Variable - Name

#[derive(Debug, Eq, PartialEq)]
pub struct VariableName<'t> {
    raw: &'t str,
}

impl<'t> VariableName<'t> {
    const fn new(raw: &'t str) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> &str {
        self.raw
    }
}

impl<'t> TryParse<'t> for VariableList<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut parsed_variable_specifications = Self::new();
        let mut state = VariableListState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                VariableListNext::Comma if rest.starts_with(',') => {
                    state.next = VariableListNext::VarSpec;
                    state.position += 1;
                }
                VariableListNext::Comma => {
                    return Ok((state.position, parsed_variable_specifications))
                }
                VariableListNext::VarSpec => match VariableSpecification::try_parse(rest) {
                    Ok((position, variable_specification)) => {
                        parsed_variable_specifications.push(variable_specification);

                        state.next = VariableListNext::Comma;
                        state.position += position;
                    }
                    Err(err) => return Err(err),
                },
            }
        }
    }
}

#[derive(Default)]
struct VariableListState {
    next: VariableListNext,
    position: usize,
}

#[derive(Default)]
enum VariableListNext {
    Comma,
    #[default]
    VarSpec,
}

// Variable - Specification

impl<'t> TryParse<'t> for VariableSpecification<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        VariableName::try_parse(raw).and_then(|(position_a, variable_name)| {
            Option::<Modifier>::try_parse(&raw[position_a..]).and_then(|(position_b, modifier)| {
                Ok((position_a + position_b, (variable_name, modifier)))
            })
        })
    }
}

// Variable - Name

impl<'t> VariableName<'t> {
    pub const fn parse() -> impl Satisfy {
        (Ascii::new(is_variable_character_ascii), PercentEncoded)
    }
}

impl<'t> TryParse<'t> for VariableName<'t> {
    // TODO: Experiment with ordering for perf?
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut state = VariableNameState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                VariableNameNext::Dot if rest.starts_with('.') => {
                    state.position += 1;
                    state.next = VariableNameNext::VariableCharacters;
                }
                VariableNameNext::Dot => {
                    return Ok((state.position, VariableName::new(&raw[..state.position])));
                }
                VariableNameNext::VariableCharacters => match Self::parse().satisfy(rest) {
                    0 => return Err(Error::msg("varname: expected valid char(s)")),
                    n => {
                        state.position += n;
                        state.next = VariableNameNext::Dot;
                    }
                },
            }
        }
    }
}

#[derive(Default)]
struct VariableNameState {
    next: VariableNameNext,
    position: usize,
}

#[derive(Default)]
enum VariableNameNext {
    Dot,
    #[default]
    VariableCharacters,
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_variable_character_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x41'..=b'\x5a' // A..Z
        | b'\x30'..=b'\x39' // 0..9
        | b'\x5f' => true,  // _
        _ => false,
    }
}
