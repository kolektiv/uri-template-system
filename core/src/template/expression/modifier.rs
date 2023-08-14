pub mod explode;
pub mod prefix;

use anyhow::{
    Error,
    Result,
};

use crate::{
    template::expression::modifier::{
        explode::Explode,
        prefix::Prefix,
    },
    TryParse,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier<'a> {
    Explode(Explode<'a>),
    Prefix(Prefix<'a>),
}

impl<'a> TryParse<'a> for Option<Modifier<'a>> {
    fn try_parse(raw: &'a str) -> Result<(usize, Self)> {
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

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_digit(c: char) -> bool {
    match c {
        | '\u{000030}'..='\u{000039}' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_non_zero_digit(c: char) -> bool {
    match c {
        | '\u{000031}'..='\u{000039}' => true,
        _ => false,
    }
}
