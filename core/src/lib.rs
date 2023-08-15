mod common;
mod expansion;
mod template;
mod value;

use anyhow::Result;

use crate::{
    expansion::Expansion,
    template::Template,
};

// =============================================================================
// URI Template
// =============================================================================

// Traits

trait Expand<V, C> {
    fn expand(&self, output: &mut String, value: &V, context: &C);
}

trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str) -> (usize, Self);
}

trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str) -> Result<(usize, Self)>;
}

// -----------------------------------------------------------------------------

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct URITemplate<'t> {
    template: Template<'t>,
}

impl<'t> URITemplate<'t> {
    pub fn expand<'e>(&'e self, values: &'e Values) -> Expansion<'e, 't> {
        Expansion::new(&self.template, values)
    }

    pub fn parse(raw: &'t str) -> Result<Self> {
        Template::try_parse(raw).map(|(_, template)| Self { template })
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::value::{
    Value,
    Values,
};
