use anyhow::Result;

use crate::{
    codec::{
        self,
        Encoding,
    },
    expression::{
        Expression,
        Modifier,
        OpLevel2,
        OpLevel3,
        OpReserve,
        Operator,
        VarSpec,
    },
    value::{
        Value,
        Values,
    },
    Expand,
};

// =============================================================================
// Expand
// =============================================================================

// Traits

trait Encode {
    type Context;

    fn encode(&self, value: &Value, output: &mut String, context: &Self::Context);
}

// -----------------------------------------------------------------------------

// Types

pub struct Expansion {
    operator: Option<Operator>,
    prefix: Option<char>,
    infix: Option<char>,
}

// -----------------------------------------------------------------------------

// Expansions

impl Expand for Expression {
    type Context = ();

    fn expand(&self, output: &mut String, values: &Values, _: &Self::Context) -> Result<()> {
        self.1.expand(output, values, &self.0)
    }
}

impl Expand for Option<Operator> {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        match self {
            Some(operator) => operator.expand(output, values, context),
            _ => context.expand(output, values, &Expansion {
                operator: None,
                prefix: None,
                infix: Some(','),
            }),
        }
    }
}

impl Expand for Operator {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        match self {
            Self::Level2(operator) => operator.expand(output, values, context),
            Self::Level3(operator) => operator.expand(output, values, context),
            Self::Reserve(operator) => operator.expand(output, values, context),
        }
    }
}

impl Expand for OpLevel2 {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        match self {
            Self::Hash => context.expand(output, values, &Expansion {
                operator: Some(Operator::Level2(OpLevel2::Hash)),
                prefix: Some('#'),
                infix: Some(','),
            }),
            Self::Plus => context.expand(output, values, &Expansion {
                operator: Some(Operator::Level2(OpLevel2::Plus)),
                prefix: None,
                infix: Some(','),
            }),
        }
    }
}

impl Expand for OpLevel3 {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        match self {
            Self::Period => context.expand(output, values, &Expansion {
                operator: Some(Operator::Level3(OpLevel3::Period)),
                prefix: Some('.'),
                infix: Some('.'),
            }),
            Self::Slash => context.expand(output, values, &Expansion {
                operator: Some(Operator::Level3(OpLevel3::Slash)),
                prefix: Some('/'),
                infix: Some('/'),
            }),
            _ => todo!(), // TODO: Remaining Operators
        }
    }
}

impl Expand for OpReserve {
    type Context = Vec<VarSpec>;

    fn expand(
        &self,
        _output: &mut String,
        _values: &Values,
        _context: &Self::Context,
    ) -> Result<()> {
        match self {
            _ => todo!(), // TODO: Reserved Operator Handling (Failure?)
        }
    }
}

impl Expand for Vec<VarSpec> {
    type Context = Expansion;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        let mut defined = self
            .iter()
            .filter_map(|var_spec| values.get(&var_spec.0).map(|value| (var_spec, value)))
            .peekable();

        if let Some(prefix) = defined.peek().and_then(|_| context.prefix) {
            output.push(prefix);
        }

        while let Some((var_spec, value)) = defined.next() {
            context.operator.encode(value, output, &var_spec.1);

            if let Some(infix) = defined.peek().and_then(|_| context.infix) {
                output.push(infix);
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------

// Encodings

impl Encode for Option<Operator> {
    type Context = Option<Modifier>;

    fn encode(&self, value: &Value, output: &mut String, context: &Self::Context) {
        match self {
            Some(operator) => operator.encode(value, output, context),
            _ => match value {
                Value::Item(item) => codec::encode(item, output, &Encoding {
                    allow_encoded: false,
                    allow: is_unreserved,
                }),
                _ => todo!(), // TODO: Remaining Value types
            },
        }
    }
}

impl Encode for Operator {
    type Context = Option<Modifier>;

    fn encode(&self, value: &Value, output: &mut String, context: &Self::Context) {
        match self {
            Operator::Level2(operator) => operator.encode(value, output, context),
            Operator::Level3(operator) => operator.encode(value, output, context),
            _ => todo!(), // TODO: Remaining Operators
        }
    }
}

impl Encode for OpLevel2 {
    type Context = Option<Modifier>;

    fn encode(&self, value: &Value, output: &mut String, _context: &Self::Context) {
        match self {
            OpLevel2::Hash | OpLevel2::Plus => match value {
                Value::Item(item) => codec::encode(item, output, &Encoding {
                    allow_encoded: false,
                    allow: |c| is_unreserved(c) || is_reserved(c),
                }),
                _ => todo!(), // TODO: Remaining Value types
            },
        }
    }
}

impl Encode for OpLevel3 {
    type Context = Option<Modifier>;

    fn encode(&self, value: &Value, output: &mut String, _context: &Self::Context) {
        match self {
            OpLevel3::Period | OpLevel3::Slash => match value {
                Value::Item(item) => codec::encode(item, output, &Encoding {
                    allow_encoded: false,
                    allow: is_unreserved,
                }),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
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
