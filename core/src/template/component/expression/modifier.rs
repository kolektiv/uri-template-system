pub mod explode;
pub mod prefix;

use anyhow::{
    Error,
    Result,
};

use crate::{
    template::component::expression::modifier::{
        explode::Explode,
        prefix::Prefix,
    },
    TryParse,
};

// =============================================================================
// Modifier
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier<'t> {
    Explode(Explode<'t>),
    Prefix(Prefix<'t>),
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Option<Modifier<'t>> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut state = State::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                Next::Symbol if rest.starts_with('*') => {
                    return Ok((1, Some(Modifier::Explode(Explode::new(&raw[..1])))));
                }
                Next::Symbol if rest.starts_with(':') => {
                    state.position += 1;
                    state.next = Next::LeadingDigit;
                }
                Next::Symbol => {
                    return Ok((0, None));
                }
                Next::LeadingDigit if rest.starts_with(is_non_zero_digit) => {
                    state.position += 1;
                    state.next = Next::TrailingDigit;
                }
                Next::LeadingDigit => {
                    return Err(Error::msg("prefix: numeric value expected"));
                }
                Next::TrailingDigit if rest.starts_with(is_digit) => {
                    if state.position < 4 {
                        state.position += 1;
                    } else {
                        return Err(Error::msg("prefix: value from 1-9999 expected"));
                    }
                }
                Next::TrailingDigit => {
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
struct State {
    next: Next,
    position: usize,
}

#[derive(Default)]
enum Next {
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
