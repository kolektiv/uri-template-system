mod component;

use std::fmt::{
    self,
    Formatter,
};

use anyhow::Result;

use crate::{
    expansion::Expand,
    template::component::Component,
    value::Values,
    TryParse,
};

// =============================================================================
// Template
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Template<'t> {
    pub components: Vec<Component<'t>>,
    pub raw: &'t str,
}

impl<'t> Template<'t> {
    fn new(raw: &'t str, components: Vec<Component<'t>>) -> Self {
        Self { components, raw }
    }
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Template<'t> {
    fn try_parse(raw: &'t str) -> Result<(usize, Self)> {
        Vec::<Component<'t>>::try_parse(raw)
            .map(|(_, components)| (raw.len(), Self::new(raw, components)))
    }
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Template<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> fmt::Result {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, f))
    }
}
