mod expression;
mod literal;

use std::fmt::{
    self,
    Formatter,
};

use anyhow::Result;

use crate::{
    common::matcher::Matcher,
    expansion::Expand,
    template::component::{
        expression::Expression,
        literal::Literal,
    },
    value::Values,
    TryParse,
};

// =============================================================================
// Component
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub enum Component<'t> {
    Literal(Literal<'t>),
    Expression(Expression<'t>),
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Vec<Component<'t>> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        let mut parsed_components = Self::new(); // TODO: Check if a default capacity estimation improves perf
        let mut state = State::default();

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
struct State {
    position: usize,
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Component<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(expression) => expression.expand(values, f),
            Self::Literal(literal) => literal.expand(values, f),
        }
    }
}

pub trait EncodeExt {
    fn write_str_encoded(&mut self, raw: &str, matcher: &impl Matcher) -> fmt::Result;
}

impl EncodeExt for Formatter<'_> {
    fn write_str_encoded(&mut self, raw: &str, matcher: &impl Matcher) -> fmt::Result {
        let mut pos = 0;

        loop {
            let rest = &raw[pos..];

            if rest.is_empty() {
                break;
            }

            match matcher.matches(rest) {
                0 => {
                    if let Some(c) = rest.chars().next() {
                        for b in c.encode_utf8(&mut [0; 4]).bytes() {
                            self.write_fmt(format_args!("%{:02X}", b))?;
                        }

                        pos += c.len_utf8();
                    }
                }
                n => {
                    self.write_str(&rest[..n])?;

                    pos += n;
                }
            }
        }

        Ok(())
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_reserved_ascii(b: u8) -> bool {
    match b {
        _ if is_gen_delim_ascii(b) => true,
        _ if is_sub_delim_ascii(b) => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_unreserved_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x41'..=b'\x5a' // A..Z
        | b'\x30'..=b'\x39' // 0..9
        | b'\x2d'..=b'\x2e' // -, .
        | b'\x5f'           // _
        | b'\x7e' => true,  // ~
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_gen_delim_ascii(b: u8) -> bool {
    match b {
        | b'\x23'           // #
        | b'\x2f'           // /
        | b'\x3a'           // :
        | b'\x3f'           // ?
        | b'\x40'           // @
        | b'\x5b'           // [
        | b'\x5d' => true,  // ]
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_sub_delim_ascii(b: u8) -> bool {
    match b {
        | b'\x21'           // !
        | b'\x24'           // $
        | b'\x26'..=b'\x2c' // &, ', (, ), *, +, ,
        | b'\x3b'           // ;
        | b'\x3d' => true,  // =
        _ => false,
    }
}
