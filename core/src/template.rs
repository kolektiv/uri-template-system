mod common;
mod component;
mod expression;
mod literal;

use anyhow::Result;

use crate::{
    template::component::Component,
    value::Values,
    Expand,
    Parse,
    ParseRef,
};

// =============================================================================
// Template
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'a> {
    pub components: Vec<Component<'a>>,
    pub parse_ref: ParseRef<'a>,
}

impl<'a> Template<'a> {
    fn new(parse_ref: ParseRef<'a>, components: Vec<Component<'a>>) -> Self {
        Self {
            components,
            parse_ref,
        }
    }
}

impl<'a> Parse<'a> for Template<'a> {
    fn parse(raw: &'a str, _base: usize) -> Result<(usize, Self)> {
        Vec::<Component<'a>>::parse(raw, 0).map(|(_, components)| {
            let len = raw.len();
            let parse_ref = ParseRef::new(0, len - 1, raw);

            (len, Self::new(parse_ref, components))
        })
    }
}

// -----------------------------------------------------------------------------

// Expansion

impl<'a> Expand<Values, ()> for Template<'a> {
    fn expand(&self, output: &mut String, values: &Values, context: &()) {
        self.components
            .iter()
            .for_each(|component| component.expand(output, values, context));
    }
}

// -----------------------------------------------------------------------------

// Re-Exports

pub use self::{
    expression::*,
    literal::*,
};
