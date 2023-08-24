use crate::{
    string::{
        satisfy::{
            Ascii,
            PercentEncoded,
        },
        Satisfy,
    },
    template::{
        component::expression::modifier::Modifier,
        ParseError,
        TryParse,
    },
};

// =============================================================================
// Variable
// =============================================================================

// Types

#[allow(clippy::module_name_repetitions)]
pub type VariableList<'t> = Vec<VariableSpecification<'t>>;

#[allow(clippy::module_name_repetitions)]
pub type VariableSpecification<'t> = (VariableName<'t>, Option<Modifier>);

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct VariableName<'t> {
    name: &'t str,
}

impl<'t> VariableName<'t> {
    const fn new(name: &'t str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &str {
        self.name
    }
}

// -----------------------------------------------------------------------------

// Parse

// Parse - Variable List

impl<'t> TryParse<'t> for VariableList<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
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
                VariableListNext::VarSpec => {
                    match VariableSpecification::try_parse(rest, global + state.position) {
                        Ok((position, variable_specification)) => {
                            parsed_variable_specifications.push(variable_specification);

                            state.next = VariableListNext::Comma;
                            state.position += position;
                        }
                        Err(err) => return Err(err),
                    }
                }
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

// Parse - Variable Specification

impl<'t> TryParse<'t> for VariableSpecification<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        VariableName::try_parse(raw, global).and_then(|(position_a, variable_name)| {
            Option::<Modifier>::try_parse(&raw[position_a..], global + position_a)
                .map(|(position_b, modifier)| (position_a + position_b, (variable_name, modifier)))
        })
    }
}

// Parse - Variable Name

impl<'t> TryParse<'t> for VariableName<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut state = VariableNameState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                VariableNameNext::VariableCharacters => match satisfier().satisfy(rest) {
                    0 => {
                        return Err(ParseError::UnexpectedInput {
                            position: global + state.position,
                            message: "unexpected input parsing variable name".into(),
                            expected: "valid var_name characters (see: https://datatracker.ietf.org/doc/html/rfc6570#section-2.3)".into(),
                        })
                    }
                    n => {
                        state.position += n;
                        state.next = VariableNameNext::Dot;
                    }
                },
                VariableNameNext::Dot if rest.starts_with('.') => {
                    state.position += 1;
                    state.next = VariableNameNext::VariableCharacters;
                }
                VariableNameNext::Dot => {
                    return Ok((state.position, VariableName::new(&raw[..state.position])));
                }
            }
        }
    }
}

const fn satisfier() -> impl Satisfy {
    (Ascii::new(is_variable_character_ascii), PercentEncoded)
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
