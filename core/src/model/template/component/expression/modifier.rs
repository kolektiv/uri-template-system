// Modifier

use anyhow::{
    Error,
    Result,
};

use crate::action::parse::TryParse;

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier<'t> {
    Explode(Explode<'t>),
    Prefix(Prefix<'t>),
}

// Modifier - Explode

#[derive(Debug, Eq, PartialEq)]
pub struct Explode<'t> {
    raw: &'t str,
}

impl<'t> Explode<'t> {
    pub const fn new(raw: &'t str) -> Self {
        Self { raw }
    }
}

// Modifier - Prefix

#[derive(Debug, Eq, PartialEq)]
pub struct Prefix<'t> {
    length: usize,
    raw: &'t str,
}

impl<'t> Prefix<'t> {
    pub const fn new(raw: &'t str, length: usize) -> Self {
        Self { length, raw }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

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
