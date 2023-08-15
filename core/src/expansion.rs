use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::{
    template::Template,
    value::Values,
};

pub trait Expand {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result;
}

pub struct Expansion<'e, 't> {
    template: &'e Template<'t>,
    values: &'e Values,
}

impl<'e, 't> Expansion<'e, 't> {
    pub fn new(template: &'e Template<'t>, values: &'e Values) -> Self {
        Self { template, values }
    }
}

impl<'e, 't> Display for Expansion<'e, 't> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.template.expand(self.values, f)
    }
}
