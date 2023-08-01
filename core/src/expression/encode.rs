use std::sync::OnceLock;

use crate::{
    codec::{
        self,
        Encoding,
    },
    expression::{
        Modifier,
        OpLevel2,
        OpLevel3,
        Operator,
        VarSpec,
    },
    value::Value,
};

// =============================================================================
// Encode
// =============================================================================

// Traits

pub trait Encode {
    type Context;
    type Value;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context);
}
// -----------------------------------------------------------------------------

// Encode

impl Encode for Option<Operator> {
    type Context = VarSpec;
    type Value = Value;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            Some(operator) => operator.encode(value, output, context),
            _ => match value {
                Value::Item(value) => context.1.encode(value, output, unreserved()),
                _ => todo!(), // TODO: Remaining Value types
            },
        }
    }
}

impl Encode for Operator {
    type Context = VarSpec;
    type Value = Value;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            Operator::Level2(operator) => operator.encode(value, output, context),
            Operator::Level3(operator) => operator.encode(value, output, context),
            _ => unreachable!(),
        }
    }
}

impl Encode for OpLevel2 {
    type Context = VarSpec;
    type Value = Value;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            OpLevel2::Hash | OpLevel2::Plus => match value {
                Value::Item(value) => context.1.encode(value, output, reserved()),
                _ => todo!(), // TODO: Remaining Value types
            },
        }
    }
}

impl Encode for OpLevel3 {
    type Context = VarSpec;
    type Value = Value;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            OpLevel3::Period | OpLevel3::Slash => match value {
                Value::Item(value) => context.1.encode(value, output, unreserved()),
                _ => todo!(),
            },
            OpLevel3::Semicolon => match value {
                Value::Item(value) => {
                    output.push_str(&context.0);

                    if !value.is_empty() {
                        output.push('=');

                        context.1.encode(value, output, unreserved())
                    }
                }
                _ => todo!(),
            },
            OpLevel3::Question | OpLevel3::Ampersand => match value {
                Value::Item(value) => {
                    output.push_str(&context.0);
                    output.push('=');

                    context.1.encode(value, output, unreserved())
                }
                _ => todo!(),
            },
        }
    }
}

impl Encode for Option<Modifier> {
    type Context = Encoding;
    type Value = String;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            Some(modifier) => modifier.encode(value, output, context),
            _ => codec::encode(value, output, context),
        }
    }
}

impl Encode for Modifier {
    type Context = Encoding;
    type Value = String;

    fn encode(&self, value: &Self::Value, output: &mut String, context: &Self::Context) {
        match self {
            Modifier::Explode => codec::encode(value, output, context),
            Modifier::Prefix(max_len) => {
                codec::encode(&value[..(*max_len).min(value.len())], output, context)
            }
        }
    }
}

// -----------------------------------------------------------------------------

// Encodings

static RESERVED: OnceLock<Encoding> = OnceLock::new();
static UNRESERVED: OnceLock<Encoding> = OnceLock::new();

fn reserved() -> &'static Encoding {
    RESERVED.get_or_init(|| Encoding {
        allow_encoded: true,
        allow: Box::new(|c| is_unreserved(c) || is_reserved(c)),
    })
}

fn unreserved() -> &'static Encoding {
    UNRESERVED.get_or_init(|| Encoding {
        allow_encoded: false,
        allow: Box::new(is_unreserved),
    })
}

// -----------------------------------------------------------------------------

// Predicates

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_gen_delim(c: char) -> bool {
    match c {
        | '\x23'
        | '\x2f'
        | '\x3a'
        | '\x3f'
        | '\x40'
        | '\x5b'
        | '\x5d' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_sub_delim(c: char) -> bool {
    match c {
        | '\x21'
        | '\x24'
        | '\x26'..='\x2c'
        | '\x3b'
        | '\x3d' => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_reserved(c: char) -> bool {
    match c {
        _ if is_gen_delim(c) => true,
        _ if is_sub_delim(c) => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_unreserved(c: char) -> bool {
    match c {
        | '\x30'..='\x39'
        | '\x41'..='\x5a'
        | '\x61'..='\x7a'
        | '\x2d'..='\x2e'
        | '\x5f'
        | '\x7e' => true,
        _ => false,
    }
}
