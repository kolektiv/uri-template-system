mod codec;

mod template;
mod value;

use anyhow::{
    Error,
    Result,
};

use crate::template::Template;

// =============================================================================
// URI Template
// =============================================================================

// Traits

trait Expand<V, C> {
    fn expand(&self, output: &mut String, value: &V, context: &C);
}

// -----------------------------------------------------------------------------

// Types

#[derive(Debug, PartialEq)]
pub struct URITemplate {
    template: Template,
}

impl URITemplate {
    pub fn parse(input: impl Into<String>) -> Result<Self> {
        let template = Template::parse(&input.into())
            .map(|(_, template)| template)
            .map_err(|_| Error::msg("uri template parse failed"))?; // TODO: Proper Error

        Ok(Self { template: template })
    }

    pub fn expand(&self, values: &Values) -> String {
        let mut output = String::new();
        let _ = self.template.expand(&mut output, values, &());

        output
    }
}

// -----------------------------------------------------------------------------

// Re-Export

pub use self::value::{
    Value,
    Values,
};
