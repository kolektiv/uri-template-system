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
        for (i, c) in raw.char_indices() {
            match (i, c) {
                (0, '*') => return Ok((1, Some(Modifier::Explode(Explode::new(&raw[..1]))))),
                (0, ':') => continue,
                (0, _) => break,
                (1, n) if is_non_zero_digit(n) => continue,
                (5, n) if is_digit(n) => return Err(Error::msg("prefix: expected 1-9999")),
                (_, n) if is_digit(n) => continue,
                (i, _) => {
                    return Ok((
                        i,
                        Some(Modifier::Prefix(Prefix::new(
                            &raw[..i],
                            raw[1..i].parse::<usize>().unwrap(),
                        ))),
                    ))
                }
            }
        }

        Ok((0, None))
    }
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
