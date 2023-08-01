use std::sync::OnceLock;

use crate::{
    codec::Encoding,
    expression::{
        Fragment,
        OpLevel2,
        OpLevel3,
        Operator,
        Reserved,
        VarSpec,
    },
    value::Value,
    Expand,
};

// =============================================================================
// Value
// =============================================================================

// Operator

impl Expand<Value, VarSpec> for Option<Operator> {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Some(operator) => operator.expand(output, value, context),
            _ => match value {
                Value::Item(value) => context.1.expand(output, value, unreserved()),
                _ => todo!(), // TODO: Remaining Value types
            },
        }
    }
}

impl Expand<Value, VarSpec> for Operator {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            Operator::Level2(operator) => operator.expand(output, value, context),
            Operator::Level3(operator) => operator.expand(output, value, context),
            _ => unreachable!(),
        }
    }
}

// -----------------------------------------------------------------------------

// Operator Level 2

impl Expand<Value, VarSpec> for OpLevel2 {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            OpLevel2::Fragment(operator) => operator.expand(output, value, context),
            OpLevel2::Reserved(operator) => operator.expand(output, value, context),
        }
    }
}

impl Expand<Value, VarSpec> for Fragment {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.1.expand(output, value, reserved()),
            _ => todo!(),
        }
    }
}

impl Expand<Value, VarSpec> for Reserved {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match value {
            Value::Item(value) => context.1.expand(output, value, reserved()),
            _ => todo!(),
        }
    }
}

// -----------------------------------------------------------------------------

// Operator Level 3

impl Expand<Value, VarSpec> for OpLevel3 {
    fn expand(&self, output: &mut String, value: &Value, context: &VarSpec) {
        match self {
            OpLevel3::Label | OpLevel3::Path => match value {
                Value::Item(value) => context.1.expand(output, value, unreserved()),
                _ => todo!(),
            },
            OpLevel3::PathParameter => match value {
                Value::Item(value) => {
                    output.push_str(&context.0);

                    if !value.is_empty() {
                        output.push('=');

                        context.1.expand(output, value, unreserved())
                    }
                }
                _ => todo!(),
            },
            OpLevel3::Query | OpLevel3::QueryContinuation => match value {
                Value::Item(value) => {
                    output.push_str(&context.0);
                    output.push('=');

                    context.1.expand(output, value, unreserved())
                }
                _ => todo!(),
            },
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
