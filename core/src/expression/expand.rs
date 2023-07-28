use anyhow::Result;

use crate::{
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
            _ => todo!(),
        }
    }
}

impl Expand for OpLevel3 {
    type Context = Vec<VarSpec>;

    fn expand(
        &self,
        _output: &mut String,
        _values: &Values,
        _context: &Self::Context,
    ) -> Result<()> {
        match self {
            _ => todo!(),
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
            _ => todo!(),
        }
    }
}

impl Expand for Vec<VarSpec> {
    type Context = Expansion;

    fn expand(&self, output: &mut String, values: &Values, context: &Self::Context) -> Result<()> {
        if let Some(prefix) = context.prefix {
            output.push(prefix);
        }

        let last_index = self.len() - 1;

        for (index, var_spec) in self.iter().enumerate() {
            var_spec.expand(output, values, context)?;

            if index != last_index {
                if let Some(infix) = context.infix {
                    output.push(infix);
                }
            }
        }

        Ok(())
    }
}

impl Expand for VarSpec {
    type Context = Expansion;

    fn expand(&self, output: &mut String, values: &Values, _context: &Self::Context) -> Result<()> {
        match values.get(&self.0) {
            Some(Value::Item(item)) => output.push_str(&item),
            _ => todo!(),
        }

        Ok(())
    }
}
