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
    Parse,
    ParseRef,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Modifier<'a> {
    Explode(Explode<'a>),
    Prefix(Prefix<'a>),
}

impl<'a> Parse<'a> for Option<Modifier<'a>> {
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Symbol if raw[state.position..].starts_with('*') => {
                    let len = 1;
                    let parse_ref = ParseRef::new(base, base, &raw[..1]);

                    return Ok((len, Some(Modifier::Explode(Explode::new(parse_ref)))));
                }
                Next::Symbol if raw[state.position..].starts_with(':') => {
                    state.position += 1;
                    state.next = Next::OneToNine;
                }
                Next::Symbol => return Ok((0, None)),
                Next::OneToNine => match raw[state.position..].chars().next() {
                    Some(i) if is_non_zero_digit(i) => {
                        state.position += 1;
                        state.next = Next::ZeroToNine;
                    }
                    _ => return Err(Error::msg("prefix: expected char 1-9")),
                },
                Next::ZeroToNine => match raw[state.position..].chars().next() {
                    Some(i) if is_digit(i) => {
                        if state.position < 4 {
                            state.position += 1;
                        } else {
                            return Err(Error::msg("prefix: expected value in range 1-9999"));
                        }
                    }
                    _ => {
                        let len = state.position;
                        let parse_ref = ParseRef::new(base, base + len - 1, &raw[..state.position]);

                        return Ok((
                            len,
                            Some(Modifier::Prefix(Prefix::new(
                                parse_ref,
                                raw[1..state.position].parse::<usize>().unwrap(),
                            ))),
                        ));
                    }
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
    #[default]
    Symbol,
    OneToNine,
    ZeroToNine,
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
