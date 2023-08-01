use crate::{
    expression::{
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
// Values
// =============================================================================

// Types

struct Expansion {
    operator: Option<Operator>,
    prefix: Option<char>,
    infix: Option<char>,
}

// -----------------------------------------------------------------------------

// Expansion

impl Expand<Values, ()> for Expression {
    fn expand(&self, output: &mut String, values: &Values, _: &()) {
        self.1.expand(output, values, &self.0);
    }
}

impl Expand<Values, Expansion> for Vec<VarSpec> {
    fn expand(&self, output: &mut String, values: &Values, context: &Expansion) {
        let mut defined = self
            .iter()
            .filter_map(|var_spec| values.get(&var_spec.0).map(|value| (var_spec, value)))
            .peekable();

        if let Some(prefix) = defined.peek().and_then(|_| context.prefix) {
            output.push(prefix);
        }

        while let Some((var_spec, value)) = defined.next() {
            context.operator.expand(output, value, &var_spec);

            if let Some(infix) = defined.peek().and_then(|_| context.infix) {
                output.push(infix);
            }
        }
    }
}

impl Expand<Values, Vec<VarSpec>> for Option<Operator> {
    fn expand(&self, output: &mut String, values: &Values, context: &Vec<VarSpec>) {
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

impl Expand<Values, Vec<VarSpec>> for Operator {
    fn expand(&self, output: &mut String, values: &Values, context: &Vec<VarSpec>) {
        match self {
            Self::Level2(operator) => operator.expand(output, values, context),
            Self::Level3(operator) => operator.expand(output, values, context),
            Self::Reserve(_operator) => unreachable!(),
        }
    }
}

impl Expand<Values, Vec<VarSpec>> for OpLevel2 {
    fn expand(&self, output: &mut String, values: &Values, context: &Vec<VarSpec>) {
        let (operator, prefix, infix) = match self {
            Self::Hash => (OpLevel2::Hash, Some('#'), Some(',')),
            Self::Plus => (OpLevel2::Plus, None, Some(',')),
        };

        context.expand(output, values, &Expansion {
            operator: Some(Operator::Level2(operator)),
            prefix,
            infix,
        });
    }
}

impl Expand<Values, Vec<VarSpec>> for OpLevel3 {
    fn expand(&self, output: &mut String, values: &Values, context: &Vec<VarSpec>) {
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
        });
    }
}
