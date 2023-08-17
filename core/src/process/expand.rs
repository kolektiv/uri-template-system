use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::model::{
    template::Template,
    value::Values,
};

// =============================================================================
// Expand
// =============================================================================

// Traits

pub trait Expand {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result;
}

// -----------------------------------------------------------------------------

// Types

pub struct Expansion<'e, 't> {
    template: &'e Template<'t>,
    values: &'e Values,
}

impl<'e, 't> Expansion<'e, 't> {
    pub(crate) const fn new(template: &'e Template<'t>, values: &'e Values) -> Self {
        Self { template, values }
    }
}

impl<'e, 't> Display for Expansion<'e, 't> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Expand::expand(self.template, self.values, f)
    }
}
