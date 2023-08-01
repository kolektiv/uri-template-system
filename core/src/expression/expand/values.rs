use crate::{
    expression::{
        Expression,
        Fragment,
        OpLevel2,
        OpLevel3,
        Operator,
        Reserved,
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
            Self::Fragment(Fragment) => (OpLevel2::Fragment(Fragment), Some('#'), Some(',')),
            Self::Reserved(Reserved) => (OpLevel2::Reserved(Reserved), None, Some(',')),
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
            Self::Label => (OpLevel3::Label, '.', '.'),
            Self::Path => (OpLevel3::Path, '/', '/'),
            Self::PathParameter => (OpLevel3::PathParameter, ';', ';'),
            Self::Query => (OpLevel3::Query, '?', '&'),
            Self::QueryContinuation => (OpLevel3::QueryContinuation, '&', '&'),
        };

        context.expand(output, values, &Expansion {
            operator: Some(Operator::Level3(operator)),
            prefix: Some(prefix),
            infix: Some(infix),
        });
    }
}
