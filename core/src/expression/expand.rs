use anyhow::{
    Error,
    Result,
};

use crate::{
    expression::{
        encode::Encode,
        Expression,
        OpLevel2,
        OpLevel3,
        Operator,
        VarSpec,
    },
    value::Values,
    Expand,
};

// =============================================================================
// Expand
// =============================================================================

// Types

pub struct Expansion {
    operator: Option<Operator>,
    prefix: Option<char>,
    infix: Option<char>,
}

// -----------------------------------------------------------------------------

// Expand

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
            Self::Reserve(_operator) => Err(Error::msg("unsupported reserved operator")),
        }
    }
}

impl Expand for OpLevel2 {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        let (operator, prefix, infix) = match self {
            Self::Hash => (OpLevel2::Hash, Some('#'), Some(',')),
            Self::Plus => (OpLevel2::Plus, None, Some(',')),
        };

        context.expand(output, values, &Expansion {
            operator: Some(Operator::Level2(operator)),
            prefix,
            infix,
        })
    }
}

impl Expand for OpLevel3 {
    type Context = Vec<VarSpec>;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        let (operator, prefix, infix) = match self {
            Self::Period => (OpLevel3::Period, '.', '.'),
            Self::Slash => (OpLevel3::Slash, '/', '/'),
            Self::Semicolon => (OpLevel3::Semicolon, ';', ';'),
            Self::Question => (OpLevel3::Question, '?', '&'),
            Self::Ampersand => (OpLevel3::Ampersand, '&', '&'),
        };

        context.expand(output, values, &Expansion {
            operator: Some(Operator::Level3(operator)),
            prefix: Some(prefix),
            infix: Some(infix),
        })
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
            context.operator.encode(value, output, &var_spec);

            if let Some(infix) = defined.peek().and_then(|_| context.infix) {
                output.push(infix);
            }
        }

        Ok(())
    }
}
