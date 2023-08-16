// Traits

use anyhow::{
    Error,
    Result,
};

use crate::{
    satisfy::{
        Ascii,
        PercentEncoded,
        Satisfier,
        Unicode,
    },
    Component,
    Explode,
    Expression,
    Fragment,
    Label,
    Literal,
    Modifier,
    OpLevel2,
    OpLevel3,
    Operator,
    Path,
    PathParameter,
    Prefix,
    Query,
    QueryContinuation,
    Reserved,
    Template,
    VariableList,
    VariableName,
    VariableSpecification,
};

// =============================================================================
// Parse
// =============================================================================

// Traits

pub trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str) -> (usize, Self);
}

pub trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str) -> Result<(usize, Self)>;
}

// =============================================================================
// Implementation
// =============================================================================

// Template

impl<'t> TryParse<'t> for Template<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        Vec::<Component<'t>>::try_parse(raw)
            .map(|(_, components)| (raw.len(), Self::new(raw, components)))
    }
}

// -----------------------------------------------------------------------------

// Component

impl<'t> TryParse<'t> for Vec<Component<'t>> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut parsed_components = Self::new(); // TODO: Check if a default capacity estimation improves perf
        let mut state = ComponentState::default();

        loop {
            let rest = &raw[state.position..];

            if rest.is_empty() {
                break;
            }

            let parsed = if rest.starts_with('{') {
                Expression::try_parse(rest).map(|(pos, expr)| (pos, Component::Expression(expr)))
            } else {
                Literal::try_parse(rest).map(|(pos, lit)| (pos, Component::Literal(lit)))
            };

            match parsed {
                Ok((pos, comp)) => {
                    parsed_components.push(comp);
                    state.position += pos;
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

// -----------------------------------------------------------------------------

// Expression

impl<'t> TryParse<'t> for Expression<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
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
                    return Err(Error::msg("expr: expected opening brace"))
                }
                ExpressionNext::Operator => match Option::<Operator>::parse(rest) {
                    (position, operator) => {
                        parsed_operator = operator;
                        state.next = ExpressionNext::VariableList;
                        state.position += position;
                    }
                },
                ExpressionNext::VariableList => match VariableList::try_parse(rest) {
                    Ok((position, variable_list)) => {
                        parsed_variable_list.extend(variable_list);
                        state.next = ExpressionNext::ClosingBrace;
                        state.position += position;
                    }
                    Err(err) => return Err(err),
                },
                ExpressionNext::ClosingBrace if rest.starts_with('}') => {
                    state.position += 1;

                    return Ok((
                        state.position,
                        Self::new(
                            &raw[..state.position],
                            parsed_operator,
                            parsed_variable_list,
                        ),
                    ));
                }
                ExpressionNext::ClosingBrace => {
                    return Err(Error::msg("exp: expected closing brace"))
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

// -----------------------------------------------------------------------------

// Operator

#[rustfmt::skip]
impl<'t> Parse<'t> for Option<Operator<'t>> {
    fn parse(raw: &'t str) -> (usize, Self) {
        raw.chars().next().and_then(|c| {
            let operator = match c {
                '+' => Some(Operator::Level2(OpLevel2::Reserved(Reserved::new(&raw[..1])))),
                '#' => Some(Operator::Level2(OpLevel2::Fragment(Fragment::new(&raw[..1])))),
                '.' => Some(Operator::Level3(OpLevel3::Label(Label::new(&raw[..1])))),
                '/' => Some(Operator::Level3(OpLevel3::Path(Path::new(&raw[..1])))),
                ';' => Some(Operator::Level3(OpLevel3::PathParameter(PathParameter::new(&raw[..1])))),
                '?' => Some(Operator::Level3(OpLevel3::Query(Query::new(&raw[..1])))),
                '&' => Some(Operator::Level3(OpLevel3::QueryContinuation(QueryContinuation::new(&raw[..1])))),
                _ => None,
            };

            operator.map(|operator| (1, Some(operator)))
        })
        .unwrap_or((0, None))
    }
}

// -----------------------------------------------------------------------------

// Variable

// Variable - List

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
    pub const fn parse() -> impl Satisfier {
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
                VariableNameNext::VariableCharacters => match Self::parse().satisfies(rest) {
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

// -----------------------------------------------------------------------------

// Modifier

impl<'t> TryParse<'t> for Option<Modifier<'t>> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut state = ModifierState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                ModifierNext::Symbol if rest.starts_with('*') => {
                    return Ok((1, Some(Modifier::Explode(Explode::new(&raw[..1])))));
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
                    return Err(Error::msg("prefix: numeric value expected"));
                }
                ModifierNext::TrailingDigit if rest.starts_with(is_digit) => {
                    if state.position < 4 {
                        state.position += 1;
                    } else {
                        return Err(Error::msg("prefix: value from 1-9999 expected"));
                    }
                }
                ModifierNext::TrailingDigit => {
                    return Ok((
                        state.position,
                        Some(Modifier::Prefix(Prefix::new(
                            &raw[..state.position],
                            raw[1..state.position].parse::<usize>().unwrap(),
                        ))),
                    ))
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

// -----------------------------------------------------------------------------

// Literal

impl<'t> Literal<'t> {
    const fn parse() -> impl Satisfier {
        (
            Ascii::new(is_literal_ascii),
            PercentEncoded,
            Unicode::new(is_literal_unicode),
        )
    }
}

impl<'t> TryParse<'t> for Literal<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        match Self::parse().satisfies(raw) {
            0 => Err(Error::msg("lit: expected valid char(s)")),
            n => Ok((n, Literal::new(&raw[..n]))),
        }
    }
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
