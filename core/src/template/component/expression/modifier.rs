use crate::{
    ParseError,
    TryParse,
};

// =============================================================================
// Modifier
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier {
    Explode,
    Prefix(usize),
}

// -----------------------------------------------------------------------------

// Parse

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
