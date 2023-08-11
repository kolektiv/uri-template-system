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
};

// =============================================================================
// Template
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'a> {
    pub components: Vec<Component<'a>>,
    pub raw: &'a str,
}

impl<'a> Template<'a> {
    fn new(raw: &'a str, components: Vec<Component<'a>>) -> Self {
        Self { components, raw }
    }
}

impl<'a> Parse<'a> for Template<'a> {
    fn parse(raw: &'a str) -> Result<(usize, Self)> {
        Vec::<Component<'a>>::parse(raw)
            .map(|(_, components)| (raw.len(), Self::new(raw, components)))
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
