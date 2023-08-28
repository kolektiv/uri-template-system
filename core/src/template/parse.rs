use thiserror::Error;

use crate::{
    string::satisfy::{
        Ascii,
        PercentEncoded,
        Satisfy,
        Unicode,
    },
    template::{
        Component,
        Expression,
        Literal,
        Modifier,
        OpLevel2,
        OpLevel3,
        Operator,
        Template,
        VariableList,
        VariableName,
        VariableSpecification,
    },
};

// =============================================================================
// Parse
// =============================================================================

// Traits

#[allow(clippy::module_name_repetitions)]
pub trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str, global: usize) -> (usize, Self);
}

#[allow(clippy::module_name_repetitions)]
pub trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str, base: usize) -> Result<(usize, Self), ParseError>;
}

// -----------------------------------------------------------------------------

// Errors

/// An [`Error`](std::error::Error) compatible type which may be the result of a
/// failure of [`Template::parse`], likely due to an invalid URI Template format
/// (as defined by the grammar given in [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570)).
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum ParseError {
    /// The input given contained an unexpected value according to the URI
    /// Template value grammar, causing parsing to fail. See the grammar at
    /// [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570) for the definition of a
    /// valid URI Template.
    #[error("{message} at position: {position}. expected: {expected}.")]
    UnexpectedInput {
        /// The position (in bytes) of the input at which the unexpected input
        /// occurs.
        position: usize,
        /// A message giving more detail about which grammatical element failed
        /// to parse the given input.
        message: String,
        /// An indication of what (valid) input was expected by the parser.
        expected: String,
    },
}

// -----------------------------------------------------------------------------

// Implementations

// Template

impl<'t> TryParse<'t> for Template<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        Vec::<Component<'t>>::try_parse(raw, global)
            .map(|(position, components)| (position, Self::new(components)))
    }
}

// Component

impl<'t> TryParse<'t> for Vec<Component<'t>> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut parsed_components = Self::new(); // TODO: Check if a default capacity estimation improves perf
        let mut state = ComponentState::default();

        loop {
            let rest = &raw[state.position..];

            if rest.is_empty() {
                break;
            }

            let parsed = if rest.starts_with('{') {
                Expression::try_parse(rest, global + state.position)
                    .map(|(position, expression)| (position, Component::Expression(expression)))
            } else {
                Literal::try_parse(rest, global + state.position)
                    .map(|(position, literal)| (position, Component::Literal(literal)))
            };

            match parsed {
                Ok((position, component)) => {
                    parsed_components.push(component);
                    state.position += position;
                }
                Err(err) => return Err(err),
            }
        }

        Ok((raw.len(), parsed_components))
    }
}

#[derive(Default)]
struct ComponentState {
    position: usize,
}

// Expression

impl<'t> TryParse<'t> for Expression<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut parsed_operator = None;
        let mut parsed_variable_list = Vec::new();
        let mut state = ExpressionState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                ExpressionNext::OpeningBrace if rest.starts_with('{') => {
                    state.next = ExpressionNext::Operator;
                    state.position += 1;
                }
                ExpressionNext::OpeningBrace => {
                    return Err(ParseError::UnexpectedInput {
                        position: global + state.position,
                        message: "unexpected input when parsing expression component".into(),
                        expected: "opening brace ('{')".into(),
                    });
                }
                ExpressionNext::Operator => {
                    let (position, operator) =
                        Option::<Operator>::parse(rest, global + state.position);

                    parsed_operator = operator;
                    state.next = ExpressionNext::VariableList;
                    state.position += position;
                }
                ExpressionNext::VariableList => {
                    match VariableList::try_parse(rest, global + state.position) {
                        Ok((position, variable_list)) => {
                            parsed_variable_list.extend(variable_list);
                            state.next = ExpressionNext::ClosingBrace;
                            state.position += position;
                        }
                        Err(err) => return Err(err),
                    }
                }
                ExpressionNext::ClosingBrace if rest.starts_with('}') => {
                    state.position += 1;

                    return Ok((
                        state.position,
                        Self::new(parsed_operator, parsed_variable_list),
                    ));
                }
                ExpressionNext::ClosingBrace => {
                    return Err(ParseError::UnexpectedInput {
                        position: global + state.position,
                        message: "unexpected input when parsing expression component".into(),
                        expected: "closing brace ('}')".into(),
                    });
                }
            }
        }
    }
}

#[derive(Default)]
struct ExpressionState {
    next: ExpressionNext,
    position: usize,
}

#[derive(Default)]
enum ExpressionNext {
    #[default]
    OpeningBrace,
    Operator,
    VariableList,
    ClosingBrace,
}

// Variables

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

impl<'t> TryParse<'t> for VariableSpecification<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        VariableName::try_parse(raw, global).and_then(|(position_a, variable_name)| {
            Option::<Modifier>::try_parse(&raw[position_a..], global + position_a)
                .map(|(position_b, modifier)| (position_a + position_b, (variable_name, modifier)))
        })
    }
}

impl<'t> TryParse<'t> for VariableName<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut state = VariableNameState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                VariableNameNext::VariableCharacters => match variable_name_satisfier().satisfy(rest) {
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

const fn variable_name_satisfier() -> impl Satisfy {
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

impl<'t> Parse<'t> for Option<Operator> {
    fn parse(raw: &'t str, _global: usize) -> (usize, Self) {
        raw.chars()
            .next()
            .and_then(|c| {
                let operator = match c {
                    '+' => Some(Operator::Level2(OpLevel2::Reserved)),
                    '#' => Some(Operator::Level2(OpLevel2::Fragment)),
                    '.' => Some(Operator::Level3(OpLevel3::Label)),
                    '/' => Some(Operator::Level3(OpLevel3::Path)),
                    ';' => Some(Operator::Level3(OpLevel3::PathParameter)),
                    '?' => Some(Operator::Level3(OpLevel3::Query)),
                    '&' => Some(Operator::Level3(OpLevel3::QueryContinuation)),
                    _ => None,
                };

                operator.map(|operator| (1, Some(operator)))
            })
            .unwrap_or((0, None))
    }
}

// Modifier

impl<'t> TryParse<'t> for Option<Modifier> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut state = ModifierState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                ModifierNext::Symbol if rest.starts_with('*') => {
                    return Ok((1, Some(Modifier::Explode)));
                }
                ModifierNext::Symbol if rest.starts_with(':') => {
                    state.position += 1;
                    state.next = ModifierNext::LeadingDigit;
                }
                ModifierNext::Symbol => {
                    return Ok((0, None));
                }
                ModifierNext::LeadingDigit if rest.starts_with(is_non_zero_digit) => {
                    state.position += 1;
                    state.next = ModifierNext::TrailingDigit;
                }
                ModifierNext::LeadingDigit => {
                    return Err(ParseError::UnexpectedInput {
                        position: global + state.position,
                        message: "unexpected input while parsing prefix modifier - invalid character".into(),
                        expected: "leading integer 1-9 (see: https://datatracker.ietf.org/doc/html/rfc6570#section-2.4.1)".into(),
                    });
                }
                ModifierNext::TrailingDigit if rest.starts_with(is_digit) => {
                    if state.position < 4 {
                        state.position += 1;
                    } else {
                        return Err(ParseError::UnexpectedInput {
                            position: global + state.position,
                            message: "unexpected input while parsing prefix modifier - out of range".into(),
                            expected: "positive integer < 10000 (see: https://datatracker.ietf.org/doc/html/rfc6570#section-2.4.1)".into(),
                        });
                    }
                }
                ModifierNext::TrailingDigit => {
                    return Ok((
                        state.position,
                        Some(Modifier::Prefix(
                            raw[1..state.position].parse::<usize>().unwrap(),
                        )),
                    ));
                }
            }
        }
    }
}

#[derive(Default)]
struct ModifierState {
    next: ModifierNext,
    position: usize,
}

#[derive(Default)]
enum ModifierNext {
    #[default]
    Symbol,
    LeadingDigit,
    TrailingDigit,
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_digit(c: char) -> bool {
    match c {
        | '\x30'..='\x39' => true, // 0..9
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_non_zero_digit(c: char) -> bool {
    match c {
        | '\x31'..='\x39' => true, // 1..9
        _ => false,
    }
}

// Literal

impl<'t> TryParse<'t> for Literal<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        match literal_satisfier().satisfy(raw) {
            0 => Err(ParseError::UnexpectedInput {
                position: global,
                message: "unexpected input parsing literal component".into(),
                expected: "valid literal characters (see: https://datatracker.ietf.org/doc/html/rfc6570#section-2.1)".into(),
            }),
            n => Ok((n, Literal::new(&raw[..n]))),
        }
    }
}

const fn literal_satisfier() -> impl Satisfy {
    (
        Ascii::new(is_literal_ascii),
        PercentEncoded,
        Unicode::new(is_literal_unicode),
    )
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_literal_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x3f'..=b'\x5b' // ?, @, A..Z, [
        | b'\x26'..=b'\x3b' // &, ', (, ),*, +, ,, -, -, ., /, 0..9, :, ;,
        | b'\x21'           // !
        | b'\x23'..=b'\x24' // #, $
        | b'\x3d'           // =
        | b'\x5d'           // ]
        | b'\x5f'           // _
        | b'\x7e' => true,  // ~
        _ => false
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_literal_unicode(c: char) -> bool {
    match c {
        | '\u{0000a0}'..='\u{00d7ff}' // ucschar...
        | '\u{00f900}'..='\u{00fdcf}'
        | '\u{00fdf0}'..='\u{00ffef}'
        | '\u{010000}'..='\u{01fffd}'
        | '\u{020000}'..='\u{02fffd}'
        | '\u{030000}'..='\u{03fffd}'
        | '\u{040000}'..='\u{04fffd}'
        | '\u{050000}'..='\u{05fffd}'
        | '\u{060000}'..='\u{06fffd}'
        | '\u{070000}'..='\u{07fffd}'
        | '\u{080000}'..='\u{08fffd}'
        | '\u{090000}'..='\u{09fffd}'
        | '\u{0a0000}'..='\u{0afffd}'
        | '\u{0b0000}'..='\u{0bfffd}'
        | '\u{0c0000}'..='\u{0cfffd}'
        | '\u{0d0000}'..='\u{0dfffd}'
        | '\u{0e0000}'..='\u{0efffd}'
        | '\u{00e000}'..='\u{00f8ff}' // iprivate...
        | '\u{0f0000}'..='\u{0ffffd}'
        | '\u{100000}'..='\u{10fffd}' => true,
        _ => false,
    }
}
