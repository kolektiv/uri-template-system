use anyhow::{
    Error,
    Result,
};

use crate::{
    Parse,
    ParseRef,
};

#[derive(Debug, Eq, PartialEq)]
pub struct VarName<'a> {
    parse_ref: ParseRef<'a>,
}

impl<'a> VarName<'a> {
    const fn new(parse_ref: ParseRef<'a>) -> Self {
        Self { parse_ref }
    }

    pub fn value(&self) -> &str {
        self.parse_ref.slice
    }
}

impl<'a> Parse<'a> for VarName<'a> {
    // TODO: Experiment with ordering for perf?
    fn parse(raw: &'a str, base: usize) -> Result<(usize, Self)> {
        let mut state = State::default();

        loop {
            match &state.next {
                Next::Dot if raw[state.position..].starts_with('.') => {
                    state.position += 1;
                    state.next = Next::VarChars;
                }
                Next::Dot => {
                    let len = state.position;
                    let parse_ref = ParseRef::new(base, base + len - 1, &raw[..len]);

                    return Ok((len, VarName::new(parse_ref)));
                }
                Next::VarChars => {
                    for (i, c) in raw[state.position..].char_indices() {
                        match &state.inner.next {
                            NextInner::VarChar if is_varchar(c) => continue,
                            NextInner::VarChar if is_percent(c) => {
                                state.inner.next = NextInner::Hex1
                            }
                            NextInner::VarChar if i > 0 => {
                                state.position += i;
                                state.next = Next::Dot;

                                break;
                            }
                            NextInner::Hex1 if is_hex_digit(c) => {
                                state.inner.next = NextInner::Hex2
                            }
                            NextInner::Hex2 if is_hex_digit(c) => {
                                state.inner.next = NextInner::VarChar
                            }
                            _ => {
                                return Err(Error::msg("varname: expected valid char(s)"));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct State {
    inner: StateInner,
    next: Next,
    position: usize,
}

#[derive(Default)]
struct StateInner {
    next: NextInner,
}

#[derive(Default)]
enum Next {
    Dot,
    #[default]
    VarChars,
}

#[derive(Default)]
enum NextInner {
    Hex1,
    Hex2,
    #[default]
    VarChar,
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
#[inline]
const fn is_varchar(c: char) -> bool {
    match c {
        | '\u{000030}'..='\u{000039}'           // 0-9           | ASCII Range
        | '\u{000041}'..='\u{00005a}'           // A-Z           | ASCII Range
        | '\u{00005f}'                          // _             | ASCII Range
        | '\u{000061}'..='\u{00007a}' => true,  // a-z           | ASCII Range
        _ => false,
    }
}

#[inline]
const fn is_percent(c: char) -> bool {
    c == '%'
}

#[inline]
const fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}
