use anyhow::{
    Error,
    Result,
};

use crate::{
    common::matcher::{
        Ascii,
        Matcher,
        PercentEncoded,
    },
    TryParse,
};

#[derive(Debug, Eq, PartialEq)]
pub struct VarName<'a> {
    raw: &'a str,
}

impl<'a> VarName<'a> {
    const fn new(raw: &'a str) -> Self {
        Self { raw }
    }

    pub fn value(&self) -> &str {
        self.raw
    }
}

impl<'a> TryParse<'a> for VarName<'a> {
    // TODO: Experiment with ordering for perf?
    fn try_parse(raw: &'a str) -> Result<(usize, Self)> {
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Dot if raw[state.position..].starts_with('.') => {
                    state.position += 1;
                    state.next = Next::VarChars;
                }
                Next::Dot => {
                    return Ok((state.position, VarName::new(&raw[..state.position])));
                }
                Next::VarChars => {
                    match (Ascii::new(is_varchar_ascii), PercentEncoded)
                        .matches(&raw[state.position..])
                    {
                        0 => return Err(Error::msg("varname: expected valid char(s)")),
                        n => {
                            state.position += n;
                            state.next = Next::Dot;
                        }
                    }
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
    Dot,
    #[default]
    VarChars,
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
#[inline]
fn is_varchar_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x41'..=b'\x5a' // A..Z
        | b'\x30'..=b'\x39' // 0..9
        | b'\x5f' => true,  // _
        _ => false,
    }
}
